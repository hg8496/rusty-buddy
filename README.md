# Rusty Buddy

Rusty Buddy is a command line tool that provides various utilities to assist in development, including chat support with
a backend powered by OpenAI, a tool to generate commit messages from git diffs, and      
more.

## Features

- **Chat Interface**: Engage in a conversation with an AI assistant, with the content of your project. Load or save chat
  sessions.
- **Commit Message Generator**: Automatically generate commit messages from `git diff` summaries.
- **Wish Fulfillment**: Collects files from a specified directory and creates a context for interactions with AI.
- **Tool Integration**: Use custom tools (like showing diffs, creating files and directories) to enhance the AI's
  capabilities.

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
  rusty-buddy commit-message                                                                                                                                                                                            
  ```                                                                                                                                                                                                                

- **Chat**
  
  Start a chat session with various options:
  
    - **Start a New Chat Session**
      ```bash
      rusty-buddy chat --new
      ```
      
    - **Continue the Last Chat Session**
      ```bash
      rusty-buddy chat --continue
      ```
      
    - **Load a Specific Chat Session by Name**
      ```bash
      rusty-buddy chat --load session_name
      ```
      
    - **Specify a Directory to Add to the Chat Context**
      ```bash
      rusty-buddy chat --directory ./src
      ```
      
    - **Use a Specific Persona for the Chat Session**
      ```bash
      rusty-buddy chat --persona rust
      ```                                      

  By default, if no session command is provided, a new chat is initiated. You can also set up and select different
  personas for tailored interactions.

- **Wish**

  Use the CLI to fulfill development wishes in a specified directory:
  ```bash                                                                                                                                                                                                            
  rusty-buddy wish ./src --tools                                                                                                                                                                            
  ```                                                                                                                                                                                                                

## Configuration

The Rusty Buddy CLI supports personalized configurations through a `config.toml` file. This file allows you to customize personas, models, and other aspects of Rusty Buddy.

### Configuration File Structure

The `config.toml` file follows this basic structure:

```toml
# Default persona to use if none is specified
default_persona = "rust"

# Model configurations for different functionalities
[ai]
chat_model = "gpt-4o-2024-08-06"
commit_model = "gpt-4o-2024-08-06"
wish_model = "gpt-4o-2024-08-06"

# Define various personas
[[personas]]
name = "rust"
chat_prompt = "You are an experienced Rust developer assisting a colleague with feature development and answering questions related to Rust programming."
file_types = ["rs", "md", "toml", "yml"]

[[personas]]
name = "java"
chat_prompt = "You are an experienced Java developer assisting a colleague with feature development and answering questions related to Java programming."
file_types = ["java"]

[[personas]]
name = "typescript"
chat_prompt = "You are an experienced Typescript developer assisting a colleague with feature development and answering questions related to Typescript programming."
file_types = ["ts"]
```

### Setting Up the Configuration File

1. **Locate the Configuration File:** The configuration file, `config.toml`, is typically located in the `.rusty` directory within your project root or home directory.

2. **Edit the Configuration File:** Customize the configuration by modifying or adding personas, changing default models, or specifying other options.

3. **Applying Configurations:** After saving your changes in `config.toml`, restart the Rusty Buddy CLI to apply the configurations.

## Persona Feature

The Rusty CLI supports customizable personas, allowing you to tailor chatbot interactions to your specific needs.
Personas provide context and a specific tone or style of interaction, simulating an experienced developer in your
desired programming language or environment.

### Built-in Personas

By default, Rusty-Buddy comes with several built-in personas:

- **Rust Developer**
- **Swift Developer**
- **Java Developer**
- **TypeScript Developer**

### Setting Up Custom Personas

To create and manage your own personas, you need to edit the configuration file that the Rusty Buddy uses to control its
behavior, and specify both the interaction style and the file types to be included in the context:

1. **Locate the Configuration File:** This file is called `config.toml` and is located in the `.rusty` directory,
   typically within your home directory or project root.

2. **Edit the Configuration File:** Add your custom personas in the `personas` array section of the file, defining each
   persona's `name`, `chat_prompt`, and `file_types`.

   Example:
   ```toml
   [[personas]]
   name = "python"
   chat_prompt = "You are an experienced Python developer assisting a colleague with feature development and answering questions related to Python programming."
   file_types = ["py", "md"]

   [[personas]]
   name = "go"
   chat_prompt = "You are an experienced Go developer assisting a colleague with feature development and answering questions related to Go programming."
   file_types = ["go"]
   ```

3. **Set the Default Persona:** Specify the persona you want to use by default in the `default_persona` field.

   Example:
   ```toml
   default_persona = "python"
   ```

4. **Save and Restart:** After editing, save the `config.toml` file. Restart the Rusty CLI to apply the new
   configurations.

### Using Personas

The persona specified in `default_persona` will be used automatically when you start a new chat session. You do not need
to pass any additional flags or arguments for personas when you run the CLI commands.

This approach to persona management allows you to customize how the CLI interacts with you, making it a flexible tool
that adapts to multiple programming environments and personal preferences.

### Shell Completion

Rusty Buddy CLI supports auto-completion for various shells, allowing you to complete commands, options, and arguments
easily. This can enhance your productivity and reduce errors when using the CLI.

#### Enabling Shell Completion

To enable shell completion, use the `--completion` flag with the shell you need:

For Bash:

```bash
rusty-buddy --completion=bash >> ~/.bashrc
```

For Zsh:

```zsh
rusty-buddy --completion=zsh >> ~/.zshrc
```

For Fish:

```shell
rusty-buddy --completion=fish > ~/.config/fish/completions/rusty-buddy.fish
```

For PowerShell:

```shell
rusty-buddy --completion=powershell >> $PROFILE
```

Reload your shell configuration after adding the completion script to activate it.

This feature provides tab-completion for commands and options, making it easier to use Rusty Buddy in your daily
workflow.

## Contributing

Contributions are welcome! Please fork this repository and make a pull request if you have any features, bug fixes, or
improvements you want to contribute.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For inquiries or support, please contact hg8496(mailto:hg8496@cstolz.de).                                                                                                                                      