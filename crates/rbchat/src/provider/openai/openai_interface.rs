//! This module provides an interface for communicating with the OpenAI AI backend.
//! It encapsulates the functionality required to send messages to the OpenAI API,
//! receive responses, and manage the flow of conversation within the Rusty Buddy application.
//!
//! The `OpenAIInterface` struct acts as a bridge between the chat service and the OpenAI API,
//! allowing for easy interaction with the model while maintaining session context and message history.
//!
//! ## Key Responsibilities
//!
//! - **Message Handling:** Converts application-specific message formats into the format required by the OpenAI API.
//! - **Session Management:** Retains state and context for ongoing conversations, facilitating a natural dialog flow.
//! - **Backend Integration:** Implements the `ChatBackend` trait to integrate seamlessly with other components in the chat ecosystem.
//!
//! ## Fields
//!
//! - `model`: A string that specifies the AI model used for generating chat messages.
//! - `timeout_duration`: A duration that represents the timeout for API requests.
//! - `last_call_completion_token`, `last_call_prompt_token`: Track token usage for
//!   the last API call.
//! - `overall_completion_token`, `overall_prompt_token`: Cumulative token usage metrics.
//!
//! ## Methods
//!
//! - `new`: Creates a new instance of `OpenAIInterface`, initializing it with the provided model and optional timeout.
//! - `send_request`: Sends a request with messages to the OpenAI backend and retrieves a response.
//! - `print_statistics`: Outputs token usage statistics related to the last request and overall usage.
//!
//! ## Using the OpenAIInterface
//!
//! The `OpenAIInterface` interacts with the OpenAI API to process chat messages.
//!
//! ## Error Handling
//!
//! All methods return a `Result`, which will contain an error of type `Box<dyn Error>` on failure.
//! Therefore, ensure to handle potential errors gracefully when invoking these methods during use.

use crate::chat::interface::{ChatBackend, Message, MessageInfo, MessageRole};
use crate::knowledge::EmbeddingService;
use crate::provider::openai::file_diff;
use crate::provider::openai::file_diff::{create_directory, create_file, update_file_section};
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionMessageToolCall, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestMessage, ChatCompletionRequestMessageContentPartImageArgs,
    ChatCompletionRequestMessageContentPartTextArgs, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, ChatCompletionRequestUserMessageContent,
    ChatCompletionRequestUserMessageContentPart, ChatCompletionResponseMessage, ChatCompletionTool,
    ChatCompletionToolArgs, ChatCompletionToolChoiceOption, ChatCompletionToolType,
    CompletionUsage, CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
    CreateChatCompletionResponse, CreateEmbeddingRequestArgs, FunctionObjectArgs, ImageDetail,
    ImageUrlArgs,
};
use async_openai::Client;
use async_trait::async_trait;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chrono::Utc;
use dotenvy::dotenv;
use log::{debug, error, info, trace, warn};
use serde_json::Value;
use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::time::Duration;
use tokio::time::timeout;

/// OpenAIInterface provides a wrapper around the OpenAI API for sending chat requests and handling tools.
/// It implements the ChatBackend trait, allowing it to be integrated into a chat application.
///
/// This struct maintains statistics related to token usage, sets a default model,
/// manages the timeout for requests, and converts messages into the format required by OpenAI's API.
#[derive(Clone)]
pub struct OpenAIInterface {
    model: String,
    timeout_duration: Duration,
    last_call_completion_token: u32,
    last_call_prompt_token: u32,
    overall_completion_token: u32,
    overall_prompt_token: u32,
    client: Client<OpenAIConfig>,
}

#[async_trait]
impl ChatBackend for OpenAIInterface {
    async fn send_request(
        &mut self,
        messages: &[Message],
        use_tools: bool,
    ) -> Result<Message, Box<dyn Error>> {
        trace!(
            "Preparing to send a request to OpenAI with messages: {:?}",
            messages
        );

        let oai_messages = self.convert_to_chat_completion_messages(messages)?;
        trace!("Converted messages: {:?}", oai_messages);

        let request = self.create_openai_request(&oai_messages, use_tools)?;
        info!(
            "Sending request to OpenAI with model '{}' and timeout {:?}.",
            self.model, self.timeout_duration
        );

        let result = timeout(self.timeout_duration, self.client.chat().create(request)).await;

        let chat_completion = match result {
            Ok(Ok(chat_completion)) => {
                info!("Received a successful response from OpenAI.");
                chat_completion
            }
            Ok(Err(e)) => {
                error!("Error while sending request to OpenAI: {:#?}", e);
                return Err(e.into());
            }
            Err(e) => {
                error!("Timeout error while waiting for OpenAI response: {:#?}", e);
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
        Ok(Message {
            role: MessageRole::Assistant,
            content,
            info: Some(MessageInfo::AssistantInfo {
                model: self.model.clone(),
                persona_name: String::new(),
                prompt_token: self.last_call_prompt_token,
                completion_token: self.last_call_completion_token,
                timestamp: Utc::now(),
            }),
        })
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

#[async_trait]
impl EmbeddingService for OpenAIInterface {
    async fn get_embedding(&self, content: Cow<'_, str>) -> Result<Box<Vec<f32>>, Box<dyn Error>> {
        info!("Generating embedding for content.");
        let truncated_content = truncate_to_max_bytes(&content, 32_000);
        let embedding_request = CreateEmbeddingRequestArgs::default()
            .model(self.model.clone())
            .input(truncated_content)
            .build()
            .unwrap();

        info!(
            "Sending embedding request to OpenAI: {:?}",
            embedding_request.model
        );
        let embedding_response = match self.client.embeddings().create(embedding_request).await {
            Ok(embedding_response) => {
                info!("Embedding created successfully.");
                embedding_response
            }
            Err(e) => {
                error!("Error creating embedding: {}", e);
                return Err(e.into());
            }
        };

        Ok(Box::new(embedding_response.data[0].embedding.clone()))
    }

    fn embedding_len(&self) -> usize {
        3072
    }
}

fn truncate_to_max_bytes(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        s
    } else {
        warn!("Truncating to {} bytes.", max_bytes);
        let mut end = max_bytes;
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        &s[..end]
    }
}

impl OpenAIInterface {
    pub fn new(model: String, timeout_secs: u64) -> Self {
        dotenv().ok();
        // Check if the OPENAI_KEY is set in the environment
        if env::var("OPENAI_KEY").is_err() {
            eprintln!("Error: The environment variable 'OPENAI_KEY' must be set.");
            std::process::exit(1);
        }

        OpenAIInterface {
            model,
            timeout_duration: Duration::from_secs(timeout_secs),
            last_call_completion_token: 0,
            last_call_prompt_token: 0,
            overall_completion_token: 0,
            overall_prompt_token: 0,
            client: OpenAIInterface::create_openai_client().unwrap(),
        }
    }

    fn convert_to_chat_completion_messages(
        &self,
        messages: &[Message],
    ) -> Result<Vec<ChatCompletionRequestMessage>, Box<dyn Error>> {
        let use_assistant_for_system_messages = self.model.starts_with("o1");

        Ok(messages
            .iter()
            .map(|msg| match msg.role {
                MessageRole::System if use_assistant_for_system_messages => {
                    ChatCompletionRequestAssistantMessageArgs::default()
                        .content(msg.content.as_str())
                        .build()
                        .unwrap()
                        .into()
                }
                MessageRole::Knowledge if use_assistant_for_system_messages => {
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
                MessageRole::Knowledge => ChatCompletionRequestSystemMessageArgs::default()
                    .content(msg.content.as_str())
                    .build()
                    .unwrap()
                    .into(),
                MessageRole::User => Self::create_user_message(msg)
                    .map_err(|_| ChatCompletionRequestUserMessageContent::default())
                    .unwrap(),
                MessageRole::Assistant => ChatCompletionRequestAssistantMessageArgs::default()
                    .content(msg.content.as_str())
                    .build()
                    .unwrap()
                    .into(),
            })
            .collect())
    }

    fn create_user_message(msg: &Message) -> Result<ChatCompletionRequestMessage, Box<dyn Error>> {
        let user_msg = ChatCompletionRequestMessageContentPartTextArgs::default()
            .text(msg.content.as_str())
            .build()?
            .into();
        let mut contents: Vec<ChatCompletionRequestUserMessageContentPart> = vec![user_msg];
        let image_path = if let Some(MessageInfo::UserInfo { image_path, .. }) = &msg.info {
            image_path.as_deref()
        } else {
            None
        };
        if let Some(image_path) = image_path {
            info!("Adding image to user message from {}", image_path);
            let image_data = std::fs::read(image_path)?;
            let base64_image = BASE64_STANDARD.encode(image_data);

            contents.push(
                ChatCompletionRequestMessageContentPartImageArgs::default()
                    .image_url(
                        ImageUrlArgs::default()
                            .url(format!("data:image/png;base64,{}", base64_image))
                            .detail(ImageDetail::High)
                            .build()?,
                    )
                    .build()?
                    .into(),
            );
        }

        Ok(ChatCompletionRequestUserMessageArgs::default()
            .content(contents)
            .build()
            .unwrap()
            .into())
    }

    fn update_statistics(&mut self, usage: CompletionUsage) {
        self.last_call_completion_token = usage.completion_tokens;
        self.last_call_prompt_token = usage.prompt_tokens;
        self.overall_completion_token += self.last_call_completion_token;
        self.overall_prompt_token += self.last_call_prompt_token;
        info!("Updated token statistics: Last call completion tokens: {}, Last call prompt tokens: {}", self.last_call_completion_token, self.last_call_prompt_token);
    }

    fn create_openai_client() -> Result<Client<OpenAIConfig>, Box<dyn Error>> {
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
                    Self::create_update_file_section_tool()?,
                ])
                .tool_choice(ChatCompletionToolChoiceOption::Required)
                .parallel_tool_calls(true);
        }
        debug!("Created request for OpenAI with tools: {}", use_tools);
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
                        "Shows the diff between an existing file and the newly generated content of that file and asks the user to apply the changes.",
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

    fn create_update_file_section_tool() -> Result<ChatCompletionTool, Box<dyn Error>> {
        Ok(ChatCompletionToolArgs::default()
            .r#type(ChatCompletionToolType::Function)
            .function(
                FunctionObjectArgs::default()
                    .name("update_file_section")
                    .description(
                        "Updates a section of a file specified by starting and ending lines with new content.",
                    )
                    .parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "The path to the file to update."
                        },
                        "start_line": {
                            "type": "integer",
                            "description": "The starting line number of the section to be updated."
                        },
                        "end_line": {
                            "type": "integer",
                            "description": "The ending line number of the section to be updated."
                        },
                        "new_content": {
                            "type": "string",
                            "description": "The new content that will replace the specified section."
                        }
                    },
                    "required": ["file_path", "start_line", "end_line", "new_content"]
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
            "update_file_section" => {
                // Handling 'update_file_section' by retrieving required arguments
                let file_path = args_json
                    .get("file_path")
                    .and_then(Value::as_str)
                    .ok_or("Missing 'file_path' argument")?;
                let start_line = args_json
                    .get("start_line")
                    .and_then(Value::as_u64) // Line indices are typically usize
                    .ok_or("Missing or invalid 'start_line' argument")?
                    as usize;
                let end_line = args_json
                    .get("end_line")
                    .and_then(Value::as_u64) // Line indices are typically usize
                    .ok_or("Missing or invalid 'end_line' argument")?
                    as usize;
                let new_content = args_json
                    .get("new_content")
                    .and_then(Value::as_str)
                    .ok_or("Missing 'new_content' argument")?;

                // Call the method to replace content within specified lines
                update_file_section(file_path, start_line, end_line, new_content).await?;
            }
            _ => {
                return Err(Box::from("Unknown tool call name."));
            }
        }
        Ok(())
    }
}
