markdown
# CLI Commands

Rusty Buddy offers a variety of command-line tools to streamline development tasks, from AI-powered interactions to file management. This guide details each command and its available options, ensuring you have the most up-to-date information to utilize Rusty Buddy effectively.

---

## Global Options

Before diving into the specific commands, Rusty Buddy provides some options that can be used globally:

- **Shell Completion Generation**

    Generate shell completion scripts for supported shells (e.g., Bash, Zsh, Fish, PowerShell).

        :::bash
        rusty-buddy --completion [bash|zsh|fish|powershell|elvish|fig|nushell|xonsh]

**Example:**

    :::bash
    rusty-buddy --completion bash > ~/.bash_completion

- **List Available Personas**

    Display a list of all available personas that can be used with the `chat` command.

        :::bash
        rusty-buddy --list-personas

---

## Commands Overview

- [`init`](#init)
- [`knowledge`](#knowledge)
- [`chat`](#chat)
- [`commit-message`](#commit-message)
- [`create-icon`](#create-icon)
- [`create-background`](#create-background)
- [`wish`](#wish)

---

## Init

### Description

Perform initial configurations, such as setting up API keys and default settings. The `init` command guides you through setting up Rusty Buddy for first-time use.

**Tip:**
The `init` command performs a recursive scan of your entire project folder, analyzing all files and subdirectories.
This comprehensive directory listing is used to recommend the most relevant persona automatically for your current project.

### Usage

You can initialize Rusty Buddy in one of two ways:

- **Automated:** (default)
`rusty-buddy init`
This will **recursively scan all files and folders in your project directory** to recommend the most suitable persona for your development environment.

- **Manual persona selection:**
`rusty-buddy init --choose-persona`
This skips the scan and provides a list of available personas for you to choose from.

### Execution Flow

1. **Choose Your AI Backend**

2. **Enter API Keys or URLs**

3. **Persona Selection**
- If you use `--choose-persona`, you will be presented with a menu of personas and can select which one to use.
- If you do not, Rusty Buddy will analyze your complete directory structure and auto-recommend a persona.

4. **Configuration Files Creation**

5. **Finish**

---

## Knowledge

### Description

Manage and interact with the Rusty Buddy knowledge database. This subcommand allows you to initialize, search, and add entries to enhance AI interactions by providing more context.

### Usage

    :::bash
    rusty-buddy knowledge <SUBCOMMAND>

### Subcommands

#### Init

Initialize the knowledge database with relevant entries from your project. This enables you to start with a smaller context and have the AI add pieces of knowledge as needed:

    :::bash
    rusty-buddy knowledge init --persona <persona_name>

- **`--persona <persona_name>`**: (Optional) Specify a persona for initialization. Defaults to a predefined persona if omitted.

#### Search

Search the knowledge database for relevant information:

    :::bash
    rusty-buddy knowledge search <search_term>

- **`<search_term>`**: The string or phrase to search within the knowledge database.

**Note:** The default output is a list of entries (e.g., file path or URL) with their similarity scores; the actual document text is *not* displayed but is available for the AI to use as context in chat.

#### Add

Expand the database by adding new entries from directories, files, or URLs:

    :::bash
    rusty-buddy knowledge add --dir <path_to_directory>

- **`--dir <path_to_directory>`**: Add all files from the specified directory to the knowledge database.

Add content from a single file or URL:

    :::bash
    rusty-buddy knowledge add --file <path_to_file>
    rusty-buddy knowledge add --url <web_url>

- **`--file <path_to_file>`**: Add a single file to the database.
- **`--url <web_url>`**: Add content from the specified URL.

### Examples

1. **Initialize with a Specific Persona**:

        :::bash
        rusty-buddy knowledge init --persona rust

    Initializes entries tailored to Rust programming expertise.

2. **Perform a Knowledge Search**:

        :::bash
        rusty-buddy knowledge search "memory management"

     Searches for entries related to memory management to aid chat sessions.

3. **Add a Directory to the Knowledge Database**:

        :::bash
        rusty-buddy knowledge add --dir ./docs/articles

     Adds all files from the `./docs/articles` directory for future use.

---

These features enable you to significantly enhance the context and responsiveness of AI interactions in Rusty Buddy, supporting more informed decisions and assistance.

---

## Chat

### Description

Engage in conversation with an AI assistant to brainstorm ideas, solve problems, or get assistance with development tasks. The chat interface supports context-aware interactions, utilizing personas and integrating directory context. Additionally, it can query a knowledge database to augment interactions with relevant information specific to the user input.

### Usage

    :::bash
    rusty-buddy chat [OPTIONS]

### Options

#### Start a New Chat Session

Start a new chat session from scratch.

    :::bash
    rusty-buddy chat --new

#### Continue the Last Chat Session

Continue the previous chat session.

    :::bash
    rusty-buddy chat --continue-last

#### Load a Specific Chat Session by Name

Load a previously saved session by name.

    :::bash
    rusty-buddy chat --load <SESSION_NAME>

#### Add an Image to the Chat (`--image/-i`)

Include an image with the chat to provide more context or visual information that the AI can analyze.

    :::bash
    rusty-buddy chat --image path/to/your/image.png

**Example:**

    :::bash
    rusty-buddy chat --image ./images/sample.png --directory ./src --knowledge

#### Fetch and Use Knowledge Before You Chat (`--knowledge/-k`)

When the `--knowledge` flag is used, the command will use the latest user input to generate embeddings and search the knowledge store for relevant documents. The relevant documents are added to the chat context before the assistant responds.

    :::bash
    rusty-buddy chat --knowledge

**Tip:** You can combine the knowledge search with any other chat options, such as adding directory contexts or specifying a persona (e.g., Rust programming persona).

**With Other Context:**

    :::bash
    rusty-buddy chat --persona rust --directory ./src --knowledge


#### Fetch Knowledge Once in a One-Shot Chat

You can also fetch knowledge before running a one-shot query to the AI.

    :::bash
    rusty-buddy chat --one-shot "Need help optimizing memory management" --knowledge

## Slash Commands in Chat

Within a chat session, you can enhance your experience with the following slash commands:

#### Renew Context

Refresh the current chat context to reload any previous interactions or settings:

    :::plaintext
    /renew

##### Save Files

Save code blocks from the assistant's last message to files.

- **Standard Mode:** The users will be prompted for each block.

        :::plaintext
        /save-files

- **Greedy Mode:** Quickly save all code blocks without user prompts.

        :::plaintext
        /save-files greedy

#### Save Last Answer

Save the entire last assistant response to a file.

    :::plaintext
    /save-last-answer

Type the slash command within the chat interface. Use `exit` to end the session, optionally saving it under a specific name.

---

#### Copy Last Message

Copy the last message from the assistant to your clipboard. This command is useful for quickly storing a response that you may want to refer back to or use in another application.

    :::plaintext
    /copy-last-message

**Key Points:**
- Copies the last assistant message directly to the clipboard using platform-specific clipboard access.
- Useful for efficiently using snippets from the conversation in other contexts or applications.

#### Copy Files

Extract and copy code blocks from the last assistant message to the clipboard.

- **Standard Mode:** Iterate over each code block, presenting the user with an option to copy.

        :::plaintext
        /copy-files

- **Greedy Mode:** Automatically copies all detected code blocks in the assistant message without prompt.

        :::plaintext
        /copy-files greedy

**Key Points:**
- Designed to streamline the process of acquiring code snippets from chat sessions.
- Can dramatically speed up the workflow when working on multiple projects or tasks that involve frequent context-switching.

---

Using these new commands, you can create a more efficient interaction loop, allowing Rusty Buddy to assist you with actionable insights and facilitating easy integration into your current workflow. By harnessing these capabilities, you greatly enhance the utility of the chat sessions, accessing and reusing information more effectively.

---

## Commit Message

### Description

Automatically generate clear and consistent commit messages from your current git diff, following best practices.

### Usage

    :::bash
    rusty-buddy commit-message

### Options

This command does not have additional options. Ensure you have staged changes with `git add` before running.

**Example:**

    :::bash
    git add .
    rusty-buddy commit-message

---

## Create Icon

### Description

Create icons based on user descriptions, utilizing AI image generation capabilities.

### Usage

    :::bash
    rusty-buddy create-icon [OPTIONS]

### Options

#### Specify Output Directory

Set the directory where generated icons will be saved.

    :::bash
    rusty-buddy create-icon --output ./icons

Option: `-o`, `--output <OUTPUT_DIR>`

#### Specify Icon Sizes

Define the sizes (in pixels) for the generated icons.

    :::bash
    rusty-buddy create-icon --sizes 64,128,256

Option: `-s`, `--sizes <SIZES>`

Default: `16,32,64,128,256,512`

#### Provide Description

If not provided via the CLI, you will be prompted to enter a description.

**With Piped Input:**

    :::bash
    echo "Design a circular blue logo" | rusty-buddy create-icon --output ./icons --sizes 64,128,256

---

## Create Background

### Description

Generate background images based on user descriptions. Supports landscape and portrait orientations.

### Usage

    :::bash
    rusty-buddy create-background [OPTIONS]

### Options

#### Specify Output File

Set the file path for the generated background image.

    :::bash
    rusty-buddy create-background --file ./backgrounds/my_background.png

Option: `-f`, `--file <FILE>`

Default: `./background.png`

#### Set Orientation

Choose the orientation of the background image.

    :::bash
    rusty-buddy create-background --orientation landscape

Option: `-o`, `--orientation [landscape|portrait]`

##### Examples

**Landscape Orientation**

    :::bash
    rusty-buddy create-background --orientation landscape --file ./backgrounds/landscape.png

- With Piped Input:

        :::bash
        echo "Create a sunset-themed background" | rusty-buddy create-background --orientation landscape --file ./backgrounds/sunset.png

**Portrait Orientation**

    :::bash
    rusty-buddy create-background --orientation portrait --file ./backgrounds/portrait.png

- With Piped Input:

        :::bash
        echo "Create a cityscape background" | rusty-buddy create-background --orientation portrait --file ./backgrounds/cityscape.png

---

## Wish

### Description

Fulfill development tasks by creating and modifying files and directories based on your instructions. The `wish` command allows you to "wish" for changes, and Rusty Buddy, using AI, will attempt to make those changes happen.

### Usage

    :::bash
    rusty-buddy wish [OPTIONS] -d <DIRECTORY>

The directory to collect files from and apply changes.

### Options

#### Specify a Directories

Enhance the session with context from a specific directory. Only relevant files are added based on the persona's capabilities.
Can be added multiple times. The respective .gitignore files will be honored.

    :::bash
    rusty-buddy wish --directory ./src --directory ./docs

Option: `-d`, `--directory <DIRECTORY>`

#### Activate Usage of Tools

Enable the usage of tools that can make changes to your filesystem (e.g., creating/updating files).

    :::bash
    rusty-buddy wish ./src --tools

Option: `-t`, `--tools`

### Examples

- **Simple Wish Command**

        :::bash
        rusty-buddy wish ./src

You'll be prompted to describe your wish.

- **Wish with Tools Enabled**

        :::bash
        rusty-buddy wish ./src --tools

Allows Rusty Buddy to create or modify files based on your instructions.

---

**For more detailed information on each command and its options, use the help flag:**

    :::bash
    rusty-buddy [COMMAND] --help

**Example:**

    :::bash
    rusty-buddy chat --help

--- 
## Troubleshooting & Error Handling

- **No Configuration/Config File Not Found**
    - Run `rusty-buddy init` to generate `.rusty/config.toml` if missing.
    - Ensure you run Rusty Buddy within or below a directory containing `.rusty/config.toml`.
- **Missing API Keys**
    - OpenAI users: Ensure `OPENAI_KEY` is set in your `.env` or exported as an environment variable.
    - Ollama users: Ensure Ollama is running and the model exists.
- **Timeout Errors**
    - If a model is taking too long, adjust `chat_timeout_secs` in `.rusty/config.toml`.
- **Shell Completion Not Working**
    - The `--completion <shell>` flag must be provided as the *very first* argument, e.g.:
      ```bash
      rusty-buddy --completion bash > ~/.bash_completion
      ```
- **Image Upload Fails**
    - Only models that support vision can process images. For OpenAI, use `gpt-4o`; for Ollama, ensure you select a model with vision support.
    - You may receive an error if you supply `--image` to a non-vision model.

For more details or if issues persist, refer to [Contact Support](mailto:hg8496@cstolz.de).

**Happy Coding with Rusty Buddy!**
