# Rusty Buddy

Rusty Buddy is a command line tool that provides various utilities to assist in development, including chat support with a backend powered by OpenAI, a tool to generate commit messages from git diffs, and      
more.

## Features
- **Chat Interface**: Engage in a conversation with an AI assistant, with the content of your project. Load or save chat sessions.
- **Commit Message Generator**: Automatically generate commit messages from `git diff` summaries.
- **Wish Fulfillment**: Collects files from a specified directory and creates a context for interactions with AI.
- **Tool Integration**: Use custom tools (like showing diffs, creating files and directories) to enhance the AI's capabilities.

## Installation
1. Clone this repository:
   ```bash                                                                                                                                                                                                           
   git clone https://github.com/hg8496/rusty-buddy.git                                                                                                                                                           
   ```                                                                                                                                                                                                               

2. Change to the project directory:
   ```bash                                                                                                                                                                                                           
   cd rusty-buddy                                                                                                                                                                                                      
   ```                                                                                                                                                                                                               

3. Build the project:
   ```bash                                                                                                                                                                                                           
   cargo build --release                                                                                                                                                                                             
   ```                                                                                                                                                                                                               

4. Make sure to set up an environment file (.env) with your OpenAI API key:
   ```plaintext                                                                                                                                                                                                      
   OPENAI_KEY=your_openai_api_key                                                                                                                                                                                    
   ```                                                                                                                                                                                                               

## Usage
### CLI Commands
- **Commit Message**

  Generate a summary for staged changes in your git repository:
  ```bash  
  git add .                                                                                                                                                                                                          
  rusty-buddy commitmessage                                                                                                                                                                                            
  ```                                                                                                                                                                                                                

- **Chat**

  Start a new chat session or continue the last one:
  ```bash                                                                                                                                                                                                            
  rusty-buddy chat --new                                                                                                                                                                                               
  rusty-buddy chat --continue                                                                                                                                                                                          
  rusty-buddy chat --load session_name                                                                                                                                                                                 
  ```                                                                                                                                                                                                                

  Add a directory context to the chat:
  ```bash                                                                                                                                                                                                            
  rusty-buddy chat --directory ./src                                                                                                                                                                        
  ```                                                                                                                                                                                                                

- **Wish**

  Use the CLI to fulfill development wishes in a specified directory:
  ```bash                                                                                                                                                                                                            
  rusty-buddy wish ./src --tools                                                                                                                                                                            
  ```                                                                                                                                                                                                                
## Persona Feature

The Rusty CLI supports customizable personas, allowing you to tailor chatbot interactions to your specific needs. Personas provide context and a specific tone or style of interaction, simulating an experienced developer in your desired programming language or environment.

### Built-in Personas

By default, Rusty-Buddy comes with several built-in personas:

- **Rust Developer**
- **Swift Developer**
- **Java Developer**
- **TypeScript Developer**

### Setting Up Custom Personas

To create and manage your own personas, you need to edit the configuration file that the Rusty CLI uses to control its behavior:

1. **Locate the Configuration File:** This file is called `config.toml` and is located in the `.rusty` directory, typically within your home directory or project root.

2. **Edit the Configuration File:** Add your custom personas in the `personas` array section of the file, defining each persona's `name` and `chat_prompt`.

   Example:
   ```toml
   [[personas]]
   name = "python"
   chat_prompt = "You are an experienced Python developer assisting a colleague with feature development and answering questions related to Python programming."

   [[personas]]
   name = "go"
   chat_prompt = "You are an experienced Go developer assisting a colleague with feature development and answering questions related to Go programming."
   ```

3. **Set the Default Persona:** Specify the persona you want to use by default in the `default_persona` field.

   Example:
   ```toml
   default_persona = "python"
   ```

4. **Save and Restart:** After editing, save the `config.toml` file. Restart the Rusty CLI to apply the new configurations.

### Using Personas

The persona specified in `default_persona` will be used automatically when you start a new chat session. You do not need to pass any additional flags or arguments for personas when you run the CLI commands.

This approach to persona management allows you to customize how the CLI interacts with you, making it a flexible tool that adapts to multiple programming environments and personal preferences.

## Contributing

Contributions are welcome! Please fork this repository and make a pull request if you have any features, bug fixes, or improvements you want to contribute.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For inquiries or support, please contact hg8496(mailto:hg8496@cstolz.de).                                                                                                                                      