use crate::knowledge::KnowledgeStore;

impl ChatService {
    // Existing methods...

    /// Adds knowledge from the knowledge database before sending a user message.
    /// Calls the knowledge store service to add relevant knowledge as messages.
    pub async fn add_knowledge(
        &mut self, 
        knowledge_store: &dyn KnowledgeStore,  // Reference to trait object
        user_input: String
    ) -> Result<(), Box<dyn Error>> {
        // Clear old knowledge messages before adding new ones
        self.messages.retain(|msg| msg.role != MessageRole::Knowledge);

        // Query the knowledge store for relevant documents based on the user input
        let knowledge_messages = knowledge_store.query_knowledge(user_input).await?;

        // Add each knowledge message to the chat session
        for knowledge_msg in knowledge_messages {
            self.messages.push(knowledge_msg);
        }

        Ok(())
    }
}