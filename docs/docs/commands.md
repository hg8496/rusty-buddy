# CLI Commands

Rusty Buddy offers a variety of command-line tools to streamline development tasks, from AI-powered interactions to file management. This guide details each command and its available options, ensuring you have the most up-to-date information to utilize Rusty Buddy effectively.

---

## Global Options

Before diving into the specific commands, Rusty Buddy provides some options that can be used globally:

- **Shell Completion Generation**

  Generate shell completion scripts for supported shells (e.g., Bash, Zsh, Fish, PowerShell).

```bash
rusty-buddy --completion [bash|zsh|fish|powershell|elvish|fig|nushell|xonsh]
```

  **Example:**

```bash
rusty-buddy --completion bash > ~/.bash_completion
```

- **List Available Personas**

  Display a list of all available personas that can be used with the `chat` command.

```bash
rusty-buddy --list-personas
```

---

## Commands Overview

- [`chat`](#chat)
- [`commit-message`](#commit-message)
- [`create-icon`](#create-icon)
- [`create-background`](#create-background)
- [`wish`](#wish)
- [`init`](#init)

---

## Chat

### Description

Engage in conversation with an AI assistant to brainstorm ideas, solve problems, or get assistance with development tasks. The chat interface supports context-aware interactions, utilizing personas and integrating directory context.

### Usage

```bash
rusty-buddy chat [OPTIONS]
```

### Options

#### Start a New Chat Session

  Begin a fresh session with the AI.

```bash
rusty-buddy chat --new
```

Option: `-n`, `--new`

#### Continue the Last Chat Session

Resume your most recent chat session.

```bash
rusty-buddy chat --continue-last
```

Option: `-c`, `--continue-last`

#### Load a Specific Chat Session by Name

Load a previously saved session.

```bash
rusty-buddy chat --load session_name
```

Option: `-l`, `--load <SESSION_NAME>`

#### Specify a Directory for Chat Context

Enhance the session with context from a specific directory. Only relevant files are added based on the persona's capabilities.

```bash
rusty-buddy chat --directory ./src
```

Option: `-d`, `--directory <DIRECTORY>`

#### Use a Specific Persona

Customize the assistant's personality based on your needs. Use `rusty-buddy --list-personas` to see available personas.

```bash
rusty-buddy chat --persona rust
```

Option: `-p`, `--persona <PERSONA_NAME>`

#### One-shot Chat Interaction

Send a one-time message and receive an immediate response. Useful for quick questions or commands.

```bash
rusty-buddy chat --one-shot "Hello, assistant!"
```

Option: `-o`, `--one-shot [<MESSAGE>]`

**With Piped Input:**

```bash
echo "What does the program do?" | rusty-buddy chat --one-shot
```

#### Specify AI Model

Choose a specific AI model for the chat session.

```bash
rusty-buddy chat --model openai_fast
```

Option: `-m`, `--model <MODEL>`

#### Silence Output of Previous Messages

When loading a session, suppress the output of previous messages.

```bash
rusty-buddy chat --continue-last --silence
```

Option: `-s`, `--silence`

### Slash Commands in Chat

Within a chat session, enhance your experience with the following slash commands:

#### Renew Context

Refresh the chat context, clearing previous interactions and reloading specified directory files.

```
/renew
```

##### Save Files

Save code blocks from the assistant's last message to files.

```
/save-files
```

Options:

- **Interactive Mode:** You'll be prompted for each code block.
- **Greedy Mode:** Quickly save all code blocks without prompts.

```
/save-files greedy
```

#### Save Last Answer

Save the entire last assistant response to a file.

```
/save-last-answer
```

Type the slash command within the chat interface. Use `exit` to end the session, optionally saving it under a specific name.

---

By using these updated options, you can maximize the efficiency and effectiveness of Rusty Buddy's chat functionality. Whether for quick responses or engaging full sessions with varied contexts, these enhancements enable comprehensive interaction with the AI capabilities.

---

## Commit Message

### Description

Automatically generate clear and consistent commit messages from your current git diff, following best practices.

### Usage

```bash
rusty-buddy commit-message
```

### Options

This command does not have additional options. Ensure you have staged changes with `git add` before running.

**Example:**

```bash
git add .
rusty-buddy commit-message
```

---

## Create Icon

### Description

Create icons based on user descriptions, utilizing AI image generation capabilities.

### Usage

```bash
rusty-buddy create-icon [OPTIONS]
```

### Options

#### Specify Output Directory

Set the directory where generated icons will be saved.

```bash
rusty-buddy create-icon --output ./icons
```

Option: `-o`, `--output <OUTPUT_DIR>`

#### Specify Icon Sizes

Define the sizes (in pixels) for the generated icons.

```bash
rusty-buddy create-icon --sizes 64,128,256
```

Option: `-s`, `--sizes <SIZES>`

Default: `16,32,64,128,256,512`

#### Provide Description

If not provided via the CLI, you will be prompted to enter a description.

**With Piped Input:**

```bash
echo "Design a circular blue logo" | rusty-buddy create-icon --output ./icons --sizes 64,128,256
```

---

## Create Background

### Description

Generate background images based on user descriptions. Supports landscape and portrait orientations.

### Usage

```bash
rusty-buddy create-background [OPTIONS]
```

### Options

#### Specify Output File

Set the file path for the generated background image.

```bash
rusty-buddy create-background --file ./backgrounds/my_background.png
```

Option: `-f`, `--file <FILE>`

Default: `./background.png`

#### Set Orientation

Choose the orientation of the background image.

```bash
rusty-buddy create-background --orientation landscape
```

Option: `-o`, `--orientation [landscape|portrait]`

**Examples:**

- **Landscape Orientation**

```bash
rusty-buddy create-background --orientation landscape --file ./backgrounds/landscape.png
```

**With Piped Input:**

```bash
echo "Create a sunset-themed background" | rusty-buddy create-background --orientation landscape --file ./backgrounds/sunset.png
```

- **Portrait Orientation**

```bash
rusty-buddy create-background --orientation portrait --file ./backgrounds/portrait.png
```

**With Piped Input:**

```bash
echo "Create a cityscape background" | rusty-buddy create-background --orientation portrait --file ./backgrounds/cityscape.png
```

---

## Wish

### Description

Fulfill development tasks by creating and modifying files and directories based on your instructions. The `wish` command allows you to "wish" for changes, and Rusty Buddy, using AI, will attempt to make those changes happen.

### Usage

```bash
rusty-buddy wish [OPTIONS] <DIRECTORY>
```

### Arguments

`<DIRECTORY>`

The directory to collect files from and apply changes.

### Options

#### Activate Usage of Tools

Enable the usage of tools that can make changes to your filesystem (e.g., creating/updating files).

```bash
rusty-buddy wish ./src --tools
```

Option: `-t`, `--tools`

### Examples

- **Simple Wish Command**

```bash
rusty-buddy wish ./src
```

You'll be prompted to describe your wish.

- **Wish with Tools Enabled**

```bash
rusty-buddy wish ./src --tools
```

Allows Rusty Buddy to create or modify files based on your instructions.

---

## Init

### Description

Perform initial configurations, such as setting up API keys and default settings. The `init` command guides you through setting up Rusty Buddy for first-time use.

### Usage

```bash
rusty-buddy init
```

### Execution Flow

1. **Choose Your AI Backend**

```plaintext
Choose backend to use (1 for OpenAI, 2 for Ollama): [User enters 1 or 2]
```

2. **Enter API Keys or URLs**

- **OpenAI:**

```plaintext
You chose OpenAI.
Please enter your OpenAI API key: [User enters key]
```

- **Ollama:**

```plaintext
You chose Ollama.
Please enter the Ollama API URL (default: http://localhost:11434): [User enters URL or presses Enter]
Please enter the Ollama model (default: llama3.2): [User enters model name or presses Enter]
```

3. **Project Analysis and Persona Recommendation**

Rusty Buddy analyzes your project and recommends a suitable persona.

```plaintext
Analyzing project directory...
Recommended persona: [Persona]
```

4. **Configuration Files Creation**

Generates `.env` and `config.toml` files in the `.rusty` directory.

---

## Additional Commands and Features

### Shell Completion

Enhance your command-line experience by enabling shell completion scripts for Rusty Buddy.

#### Generate Shell Completion Script

```bash
rusty-buddy --completion [bash|zsh|fish|powershell|elvish|fig|nushell|xonsh]
```

**Example for Bash:**

```bash
rusty-buddy --completion bash > ~/.bash_completion
source ~/.bash_completion
```

### List Available Personas

View all personas that can be used with Rusty Buddy's `chat` command.

```bash
rusty-buddy --list-personas
```

By leveraging these commands, you can greatly enhance your development workflow with the capabilities of Rusty Buddy. Whether you need to generate commit messages, engage in AI-assisted chats, or create engaging visuals, this toolset provides comprehensive support.

**For more detailed information on each command and its options, use the help flag:**

```bash
rusty-buddy [COMMAND] --help
```

**Example:**

```bash
rusty-buddy chat --help
```

--- 

**Happy Coding with Rusty Buddy!**