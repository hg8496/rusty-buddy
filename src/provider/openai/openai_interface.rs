use crate::chat::interface::{ChatBackend, Message, MessageRole};
use crate::provider::openai::file_diff;
use crate::provider::openai::file_diff::{create_directory, create_file};
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionMessageToolCall, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, ChatCompletionResponseMessage, ChatCompletionTool,
    ChatCompletionToolArgs, ChatCompletionToolChoiceOption, ChatCompletionToolType,
    CompletionUsage, CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
    CreateChatCompletionResponse, FunctionObjectArgs,
};
use async_openai::Client;
use async_trait::async_trait;
use dotenvy::dotenv;
use log::{debug, info};
use serde_json::Value;
use std::env;
use std::error::Error;
use std::time::Duration;
use tokio::time::timeout;

pub struct OpenAIInterface {
    model: String,
    timeout_duration: Duration,
    last_call_completion_token: u32,
    last_call_prompt_token: u32,
    overall_completion_token: u32,
    overall_prompt_token: u32,
}

impl Default for OpenAIInterface {
    fn default() -> Self {
        OpenAIInterface {
            model: "gpt-4o-2024-08-06".to_string(),
            last_call_completion_token: 0,
            last_call_prompt_token: 0,
            overall_completion_token: 0,
            overall_prompt_token: 0,
            timeout_duration: Duration::from_secs(30),
        }
    }
}

#[async_trait]
impl ChatBackend for OpenAIInterface {
    async fn send_request(
        &mut self,
        messages: &[Message],
        use_tools: bool,
    ) -> Result<String, Box<dyn Error>> {
        let oai_messages = self.convert_to_chat_completion_messages(messages);

        dotenv().ok();

        let client = self.create_openai_client()?;
        let request = self.create_openai_request(&oai_messages, use_tools)?;

        // Use the timeout_duration from the struct
        info!(
            "Sending request to OpenAI with timeout of {:?}.",
            self.timeout_duration
        );
        let result = timeout(self.timeout_duration, client.chat().create(request)).await;

        let chat_completion = match result {
            Ok(Ok(chat_completion)) => chat_completion,
            Ok(Err(e)) => {
                return Err(e.into());
            }
            Err(e) => {
                return Err(e.into());
            }
        };

        let usage = chat_completion.usage.clone().unwrap();
        self.update_statistics(usage);

        debug!("Extracting returned message from chat completion.");
        let returned_message = self.extract_returned_message(&chat_completion)?;

        if use_tools {
            if let Some(tool_calls) = returned_message.tool_calls {
                for tool_call in tool_calls {
                    debug!("Handling tool call: {:?}", tool_call.function.name);
                    self.handle_tool_call(tool_call).await?;
                }
            }
        }

        let content = returned_message.content.unwrap_or_default();
        info!("Request processing completed successfully.");
        Ok(content)
    }

    fn print_statistics(&self) {
        println!(
            "Last Call Completion Tokens: {}, Last Call Prompt Tokens: {}, Overall Completion Tokens: {}, Overall Prompt Tokens: {}",
            self.last_call_completion_token,
            self.last_call_prompt_token,
            self.overall_completion_token,
            self.overall_prompt_token
        );
    }
}

impl OpenAIInterface {
    pub fn new(model: String, timeout_secs: u64) -> Self {
        OpenAIInterface {
            model,
            timeout_duration: Duration::from_secs(timeout_secs),
            ..Default::default()
        }
    }

    fn convert_to_chat_completion_messages(
        &self,
        messages: &[Message],
    ) -> Vec<ChatCompletionRequestMessage> {
        let use_assistant_for_system_messages = self.model.starts_with("o1");

        messages
            .iter()
            .map(|msg| match msg.role {
                MessageRole::System if use_assistant_for_system_messages => {
                    ChatCompletionRequestAssistantMessageArgs::default()
                        .content(msg.content.as_str())
                        .build()
                        .unwrap()
                        .into()
                }
                MessageRole::Context if use_assistant_for_system_messages => {
                    ChatCompletionRequestAssistantMessageArgs::default()
                        .content(msg.content.as_str())
                        .build()
                        .unwrap()
                        .into()
                }
                MessageRole::System => ChatCompletionRequestSystemMessageArgs::default()
                    .content(msg.content.as_str())
                    .build()
                    .unwrap()
                    .into(),
                MessageRole::Context => ChatCompletionRequestSystemMessageArgs::default()
                    .content(msg.content.as_str())
                    .build()
                    .unwrap()
                    .into(),
                MessageRole::User => ChatCompletionRequestUserMessageArgs::default()
                    .content(msg.content.as_str())
                    .build()
                    .unwrap()
                    .into(),
                MessageRole::Assistant => ChatCompletionRequestAssistantMessageArgs::default()
                    .content(msg.content.as_str())
                    .build()
                    .unwrap()
                    .into(),
            })
            .collect()
    }
    fn update_statistics(&mut self, usage: CompletionUsage) {
        self.last_call_completion_token = usage.completion_tokens;
        self.last_call_prompt_token = usage.prompt_tokens;
        self.overall_completion_token += self.last_call_completion_token;
        self.overall_prompt_token += self.last_call_prompt_token;
    }

    fn create_openai_client(&self) -> Result<Client<OpenAIConfig>, Box<dyn Error>> {
        let openai_key = env::var("OPENAI_KEY")?;
        Ok(Client::with_config(
            OpenAIConfig::default().with_api_key(openai_key),
        ))
    }

    fn create_openai_request(
        &self,
        messages: &[ChatCompletionRequestMessage],
        use_tools: bool,
    ) -> Result<CreateChatCompletionRequest, Box<dyn Error>> {
        let mut builder = &mut CreateChatCompletionRequestArgs::default();
        builder = builder.model(self.model.as_str()).messages(messages);
        if use_tools {
            builder = builder
                .tools(vec![
                    Self::create_diff_tool()?,
                    Self::create_new_dir_tool()?,
                    Self::create_new_file_tool()?,
                ])
                .tool_choice(ChatCompletionToolChoiceOption::Required)
                .parallel_tool_calls(true);
        }
        Ok(builder.build()?)
    }
    fn create_new_file_tool() -> Result<ChatCompletionTool, Box<dyn Error>> {
        Ok(ChatCompletionToolArgs::default()
            .r#type(ChatCompletionToolType::Function)
            .function(
                FunctionObjectArgs::default()
                    .name("create_file")
                    .description("Creates a new file with given content at the specified path.")
                    .parameters(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "file_path": {
                                "type": "string",
                                "description": "The path for the new file."
                            },
                            "file_content": {
                                "type": "string",
                                "description": "The content to write to the new file."
                            }
                        },
                        "required": ["file_path", "file_content"]
                    }))
                    .build()?,
            )
            .build()?)
    }
    fn create_new_dir_tool() -> Result<ChatCompletionTool, Box<dyn Error>> {
        Ok(ChatCompletionToolArgs::default()
            .r#type(ChatCompletionToolType::Function)
            .function(
                FunctionObjectArgs::default()
                    .name("create_directory")
                    .description("Creates a new directory at the specified path.")
                    .parameters(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "directory_path": {
                                "type": "string",
                                "description": "The path where the new directory should be created."
                            }
                        },
                        "required": ["directory_path"]
                    }))
                    .build()?,
            )
            .build()?)
    }
    fn create_diff_tool() -> Result<ChatCompletionTool, Box<dyn Error>> {
        Ok(ChatCompletionToolArgs::default()
            .r#type(ChatCompletionToolType::Function)
            .function(
                FunctionObjectArgs::default()
                    .name("show_diff")
                    .description(
                        "Shows the diff of a file and the string of the newly generated content of that file.",
                    )
                    .parameters(serde_json::json!({
                                    "type": "object",
                                    "properties": {
                                        "diff_file": {
                                            "type": "string",
                                            "description": "The path to the original file."
                                        },
                                        "diff_content": {
                                            "type": "string",
                                            "description": "The new content of the file to display."
                                        }
                                    },
                                    "required": ["diff_file", "diff_content"]
                                }))
                    .build()?,
            )
            .build()?)
    }

    fn extract_returned_message(
        &self,
        chat_completion: &CreateChatCompletionResponse,
    ) -> Result<ChatCompletionResponseMessage, Box<dyn Error>> {
        Ok(chat_completion
            .choices
            .first()
            .ok_or("No choices returned from chat completion")?
            .message
            .clone())
    }

    async fn handle_tool_call(
        &self,
        tool_call: ChatCompletionMessageToolCall,
    ) -> Result<(), Box<dyn Error>> {
        let args_json: Value = serde_json::from_str(&tool_call.function.arguments)?;
        match tool_call.function.name.as_str() {
            "create_file" => {
                let file_path = args_json
                    .get("file_path")
                    .and_then(Value::as_str)
                    .ok_or("Missing 'file_path' argument")?;
                let file_content = args_json
                    .get("file_content")
                    .and_then(Value::as_str)
                    .ok_or("Missing 'file_content' argument")?;
                create_file(file_path, file_content).await?;
            }
            "create_directory" => {
                let directory_path = args_json
                    .get("directory_path")
                    .and_then(Value::as_str)
                    .ok_or("Missing 'directory_path' argument")?;
                create_directory(directory_path).await?;
            }
            "show_diff" => {
                let diff_file = args_json
                    .get("diff_file")
                    .and_then(Value::as_str)
                    .ok_or("Missing 'diff_file' argument")?;
                let diff_content = args_json
                    .get("diff_content")
                    .and_then(Value::as_str)
                    .ok_or("Missing 'diff_content' argument")?;
                file_diff::show_diff_in_beyond_compare(diff_file, diff_content).await?;
            }
            _ => {
                return Err(Box::from("Unknown tool call name."));
            }
        }
        Ok(())
    }
}
