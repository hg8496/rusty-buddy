# Rusty Buddy

Rusty Buddy is a command-line tool that provides various utilities to assist in development, including AI-powered interactions and file management.

## Features

- **Context-Aware Interaction**: Rusty Buddy can now build context from a project directory while respecting `.gitignore` rules, ensuring that ignored files and directories are skipped. It also supports including specific files like `Gemfile` and `Dockerfile` along with files by their extensions.

- **Chat Interface**: Engage in a conversation with an AI assistant. Load or save chat sessions, and specify a directory to include in the chat context for more relevant interactions.

- **Slash Command Auto-Completion**: Enjoy interactive auto-completion for slash commands within chat sessions. Type a `/` at the beginning of a line to see available commands, cycle through suggestions using arrow keys, and quickly complete commands with the Tab key.

- **Commit Message Generator**: Automatically generate commit messages from `git diff` summaries, ensuring clear and consistent commit histories.

- **Wish Fulfillment**: Collects files from a specified directory while adhering to `.gitignore`, creating a development context to help integrate AI into software development workflows. Utilize tools for file and directory creation and modification.

- **Icon and Background Image Generation**: Generate icons and backgrounds based on user descriptions using OpenAI's DALL·E. The commands now support piped input to streamline usage in scripts and other non-interactive environments. Specify output sizes and orientations for tailored usage:

    - **Piped Input Support**: You can provide descriptions directly via standard input (stdin), enabling seamless integration with other command-line tools.

- **Tool Integration**: Use custom tools (like showing diffs, creating files, and directories) to enhance the AI's capabilities and assist users in making swift development changes.

- **Shell Completion**: Supports shell completion for convenient command-line interaction across different shells including Bash, Zsh, Fish, and PowerShell.

## Installation

You can install Rusty Buddy by using the provided installation script, or by cloning the repository and building the project.

### Method 1: Install Using Script

For a quick installation using a script, run the following command:

```bash
curl -sSL https://get.rusty-buddy.org | bash
```

This script will download and install the Rusty Buddy binaries onto your system. Make sure you have the necessary permissions to run the installation script.

### Method 2: Clone and Build

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

## Setup

The `init` command is a setup utility designed to streamline the initial configuration process for the Rusty Buddy CLI. It ensures that your environment is correctly set up with the necessary credentials and configurations before using the tool. This command is particularly useful for first-time users or when setting up on a new system.

### Features

- **API Key Management**: Prompts for the OpenAI API key if it isn't already set in the environment or in a `.env` file. Automatically stores the key in a `.env` file for future use.
- **Automatic Persona Selection**: Analyzes the current project's directory structure and uses OpenAI's API to determine the most suitable persona based on the given directory's content.
- **Configuration Initialization**: Generates a default `config.toml` file in the `.rusty` directory, setting the stage for a personalized project setup with the selected persona.

### Usage

To run the `init` command, open your terminal and navigate to the directory where you've installed Rusty Buddy. Execute the following command:

```bash
rusty-buddy init
```

### Execution Flow

1. **OpenAI API Key Prompt**:
    - If no valid OpenAI API key is found, the command will prompt you to enter your OpenAI API key.
    - The key is stored in a `.env` file in the current directory for future reference.

2. **Persona Recommendation**:
    - The command performs a recursive directory listing, aggregating the types of files and project structure.
    - Communicates with OpenAI using the internal chat API to recommend a suitable persona based on the project's characteristics.

3. **Configuration File Creation**:
    - Generates a `config.toml` file in the `.rusty` directory with the recommended persona and assigns default AI models for chat and commit message generation.

### Example Output

After executing the `init` command, you will see:

```
OpenAI key not found in the environment.
Please enter your OpenAI API key: [User enters key]
Configuration successfully initialized with persona: [Recommended Persona]
```

### Prerequisites

- Ensure you have an OpenAI API key handy to enter when prompted.
- The command requires network access to communicate with the OpenAI servers for persona recommendation.

### Notes

- Running the `init` command is a one-time setup operation per project directory. If you switch projects or directories, you may need to rerun `rusty-buddy init`.
- The `.env` file and `.rusty/config.toml` are crucial for the CLI's correct operation, storing sensitive and configuration data safely.

By following these simple steps, you can quickly configure your development environment with Rusty Buddy for a seamless start.

## Usage

### CLI Commands

- **Create Background**

  Generate a background image using a description provided by the user. You can provide the description directly through the terminal or pipe it in:

    - **Landscape Orientation**:

      ```bash
      rusty-buddy create-background --orientation landscape --output ./backgrounds
      ```

      With piped input:

      ```bash
      echo "Create a sunset-themed background" | rusty-buddy create-background --orientation landscape --output ./backgrounds
      ```

    - **Portrait Orientation**:

      ```bash
      rusty-buddy create-background --orientation portrait --output ./backgrounds
      ```

      With piped input:

      ```bash
      echo "Create a cityscape background" | rusty-buddy create-background --orientation portrait --output ./backgrounds
      ```

- **Create Icon**

  Generate an icon using a description provided by the user. You can specify output sizes as well:

  Run interactively:

  ```bash
  rusty-buddy create-icon --output ./icons --sizes 16,32,64,128,256,512
  ```

  With piped input:

  ```bash
  echo "Design a circular blue logo" | rusty-buddy create-icon --output ./icons --sizes 64,128,256
  ```

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

    - **One-shot Chat**

      Send a single message and print the assistant's response:

        - From a specified message:
          ```bash
          rusty-buddy chat --one-shot --message "Hello, assistant!"
          ```

        - From piped input:
          ```bash
          echo "What does the program do?" | rusty-buddy chat --one-shot -d .
          ```

  By default, if no session command is provided, a new chat is initiated.

- **Wish**

  Use the CLI to fulfill development wishes in a specified directory:
  ```bash                                                                                                                                                                                                            
  rusty-buddy wish ./src --tools                                                                                                                                                                            
  ```

### Slash Commands

Within a chat session, you can use slash commands to execute specific tasks. These commands begin with a `/` character. 
After entering a `/` you can use `Tab` to cycle through possible slash commands.
The following slash commands are currently supported:

- **Renew Context**: Refresh the context of the chat session.
  ```
  /renew
  ```
  This will clear the existing context, reload files if a directory context is specified, and reapply persona prompts.

- **Save Files**: Extract code blocks from the latest assistant message and save them to a file.
  ```
  /save-files
  ```
  Options include **y** to save block or **n** to skip. For a quick save of all available code between first and last triple backticks, use:
  ```
  /save-files greedy
  ```

- **Save Last Answer**: Save the complete last response from the assistant to a file.
  ```
  /save-last-answer
  ```
  This command will store the last assistant's message into a file named `last_answer.txt`.

To execute a slash command, type it within the chat interface. For example, to save the last answer, you would enter:
```
/save-last-answer
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
commit_model = "gpt-4o-mini"
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

The Rusty CLI supports customizable personas, allowing you to tailor chatbot interactions to your specific needs. Personas provide context and a specific tone or style of interaction, simulating an experienced developer in your desired programming language or environment.

### Built-in Personas

By default, Rusty-Buddy comes with several built-in personas:

| Persona Type                        | ID                            |
|-------------------------------------|-------------------------------|
| Rust Developer                      | rust                          |
| Swift Developer                     | swift                         |
| Java Developer                      | java                          |
| TypeScript Developer                | typescript                    |
|  Embedded C Specialist              | embedded_c_specialist         |
| Yocto C++ Linux Embedded Specialist | yocto_cpp_embedded_specialist |
| Python Developer                    | python                        |
| JavaScript Developer                | javascript                    |
| C++ Developer                       | cplusplus                     |
| C# Developer                        | csharp                        |
| PHP Developer                       | php                           |
| Ruby Developer                      | ruby                          |
| Go Developer                        | golang                        |
| Kotlin Developer                    | kotlin                        |
| R Developer                         | r                             |
| Scala Developer                     | scala                         |
| Shell Scripting Developer           | shell                         |
| Perl Developer                      | perl                          |
| Dart Developer                      | dart                          |
| Objective-C Developer               | objective-c                   |
| Ansible Expert                      | ansible_expert                |
| DevOps Engineer                     | devops_engineer               |
| LaTeX Book Setter                   | latex_book_setter             |
| Technical Writer                    | technical_writer              |
| Poet                                | poet                          |
| Novelist                            | novelist                      |
| Screenwriter                        | screenwriter                  |
| Journalist                          | journalist                    |
| Content Writer                      | content_writer                |
| UX/UI Designer                      | ux_ui_designer                |
| Data Scientist                      | data_scientist                |
| Cybersecurity Analyst               | cybersecurity_analyst         |
| SEO Specialist                      | seo_specialist                |
| Social Media Manager                | social_media_manager          |
| Project Manager                     | project_manager               |
| Game Developer                      | game_developer                |
| AI/ML Engineer                      | ai_ml_engineer                |
| Digital Marketing Strategist        | digital_marketing_strategist  |

### Setting Up Custom Personas

To create and manage your own personas, you need to edit the configuration file that the Rusty Buddy uses to control its behavior, and specify both the interaction style and the file types to be included in the context:

1. **Locate the Configuration File:** This file is called `config.toml` and is located in the `.rusty` directory, typically within your home directory or project root.

2. **Edit the Configuration File:** Add your custom personas in the `personas` array section of the file, defining each persona's `name`, `chat_prompt`, and `file_types`.

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