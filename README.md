# Rusty Buddy

Rusty Buddy is a command-line tool that provides various utilities to assist in development, including AI-powered interactions and file management.

## Features

- **Chat Interface**: Engage in a conversation with an AI assistant. Load or save chat sessions, and specify a directory to include in the chat context for more relevant interactions.

- **Commit Message Generator**: Automatically generate commit messages from `git diff` summaries, ensuring clear and consistent commit histories.

- **Wish Fulfillment**: Collects files from a specified directory and creates a development context, helping integrate AI into software development workflows. Utilize tools for file and directory creation and modification.

- **Icon Generation**: Generate icons based on user descriptions using OpenAI's DALL·E. Specify output sizes for tailored icon usage.

- **Tool Integration**: Use custom tools (like showing diffs, creating files, and directories) to enhance the AI's capabilities and assist users in making swift development changes.

- **Shell Completion**: Supports shell completion for convenient command-line interaction across different shells including Bash, Zsh, Fish, and PowerShell.

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

  By default, if no session command is provided, a new chat is initiated.

- **Wish**

  Use the CLI to fulfill development wishes in a specified directory:
  ```bash
  rusty-buddy wish ./src --tools
  ```

- **Create Icon**

  Generate an icon using a user-provided description, specifying output sizes:
  ```bash
  rusty-buddy create-icon --output ./icons --sizes 16,32,64,128,256,512
  ```

### Slash Commands

Within a chat session, you can use slash commands to execute specific tasks. These commands begin with a `/` character. The following slash commands are currently supported:

- **Renew Context**: Refresh the context of the chat session.
  ```
  /renew
  ```
  This will clear the existing context, reload files if a directory context is specified, and reapply persona prompts.

- **Save Files**: Extract code blocks from the latest assistant message and save them to a file.
  ```
  /save-files
  ```
  This command allows you to interactively extract code snippets from AI responses and save them. Options include:
  - **a**: Add code block to the current memory for later saving.
  - **s**: Save accumulated code blocks to a file.
  - **q**: Quit without saving.

  For a quick save of all available code between first and last triple backticks, use:
  ```
  /save-files greedy
  ```

To execute a slash command, type it within the chat interface. For example, to save files, you would enter:
```
/save-files
```

When you're done with the chat session, typing `exit` will allow you to exit, optionally saving the chat session under a specified name.

### Examples of Chat Interactions

1. Start a new session or continue a previous session.
2. Engage in dialogue using natural language.
3. Use slash commands to adjust the session context or perform other operations.
4. Save the session if needed for future reference.

These features make the chat component of `rusty-buddy` highly customizable and user-friendly, providing tool and AI support directly from your project contexts.

## Configuration

The Rusty Buddy CLI supports personalized configurations through a `config.toml` file. This file allows you to customize personas, models, and other aspects.

### Configuration File Structure

The `config.toml` follows this structure:

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
chat_prompt = "You are an experienced TypeScript developer assisting a colleague with feature development and answering questions related to TypeScript programming."
file_types = ["ts"]
```

### Setting Up the Configuration File

1. **Locate the Configuration File:** It is typically located in the `.rusty` directory within your project root or home directory.

2. **Edit the Configuration File:** Customize the configuration by modifying or adding personas, changing default models, or specifying other options.

3. **Applying Configurations:** After saving your changes in `config.toml`, restart the Rusty Buddy CLI to apply the configurations.

## Persona Feature

Rusty CLI supports customizable personas, allowing you to tailor chatbot interactions to your specific needs. Personas provide context and a specific tone or style of interaction, simulating an experienced developer in your desired programming language or environment.

### Built-in Personas

By default, Rusty-Buddy comes with several built-in personas:

- **Rust Developer**
- **Swift Developer**
- **Java Developer**
- **TypeScript Developer**

### Setting Up Custom Personas

To create and manage your own personas, edit the configuration file that controls Rusty Buddy's behavior, specifying both the interaction style and the file types for context:

1. **Locate the Configuration File** in the `.rusty` directory.

2. **Edit the Configuration File** to include your custom personas in the `personas` array, defining each persona's `name`, `chat_prompt`, and `file_types`.

   Example:
   ```toml
   [[personas]]
   name = "python"
   chat_prompt = "You are an experienced Python developer assisting a colleague with feature development and answering questions related to Python programming."
   file_types = ["py", "md"]
   ```

3. **Set the Default Persona,** if desired, in the `default_persona` field.

   Example:
   ```toml
   default_persona = "python"
   ```

4. **Save and Restart** the Rusty CLI to apply the new configurations.

### Shell Completion

Rusty Buddy CLI supports auto-completion for various shells, enhancing productivity and reducing errors.

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
```fish
rusty-buddy --completion=fish > ~/.config/fish/completions/rusty-buddy.fish
```

For PowerShell:
```powershell
rusty-buddy --completion=powershell >> $PROFILE
```

Reload your shell configuration after adding the completion script to activate it.

## Contributing

Contributions are welcome! Please fork this repository and make a pull request if you have any features, bug fixes, or improvements you want to contribute.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For inquiries or support, please contact hg8496(mailto:hg8496@cstolz.de).
