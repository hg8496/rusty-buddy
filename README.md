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

## Contributing

Contributions are welcome! Please fork this repository and make a pull request if you have any features, bug fixes, or improvements you want to contribute.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For inquiries or support, please contact hg8496(mailto:hg8496@cstolz.de).                                                                                                                                      