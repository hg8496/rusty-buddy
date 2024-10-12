use crate::knowledge::KnowledgeStoreImpl;

pub async fn run_chat(args: ChatArgs) -> Result<(), Box<dyn Error>> {
    // Configuration setup
    let config = get_config();
    let mut chat_service = initialize_chat_service(args, config.clone())?;

    // Handle session logic (new, load existing, continue)
    handle_session(&mut chat_service, args.new, args.continue_last, &args.load)?;

    // Check if the knowledge flag is enabled
    if args.knowledge {
        // Initialize the KnowledgeStoreImpl and pass it to the chat service
        let knowledge_store = KnowledgeStoreImpl::new().await?;
        let user_last_input = get_last_user_message(&chat_service)?;  // Get the last User input

        chat_service.add_knowledge(&knowledge_store, user_last_input).await?;
    }

    // Handle normal chat flow, either one-shot or interactive
    if args.one_shot.is_some() {
        let message = args.one_shot.as_ref().unwrap();
        return handle_one_shot_mode(
            chat_service,
            message.clone(),
            config.ai.chat_model,
            resolve_persona(&args.persona, config.default_persona.as_str())?
        ).await;
    }

    start_interactive_chat(chat_service).await
}