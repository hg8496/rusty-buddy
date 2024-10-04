# CLI Commands

Rusty Buddy offers a variety of command-line tools to streamline development tasks, from AI-powered interactions to file management. This guide details each command and its available options.

## Chat

### Description:
Engage in conversation with an AI assistant to brainstorm ideas, solve problems, or get assistance with development tasks.

### Commands:

- **Start a New Chat Session**

  Begin a fresh session with the AI.

  ```bash
  rusty-buddy chat --new
  ```

- **Continue the Last Chat Session**

  Resume your most recent chat session.

  ```bash
  rusty-buddy chat --continue
  ```

- **Load a Specific Chat Session by Name**

  Load a previously saved session.

  ```bash
  rusty-buddy chat --load session_name
  ```

- **Specify a Directory for Chat Context**

Enhance the session with context from a specific directory. 
The `.gitignore` file will be honored and only required files are added.
Each Persona has its own set of files it will be looking for.

  ```bash
  rusty-buddy chat --directory ./src
  ```

- **Use a Specific Persona**

  Customize the assistant's personality based on your needs.

  ```bash
  rusty-buddy chat --persona rust
  ```

- **One-shot Chat Interaction**

  Send a message and receive an immediate response without starting a session.

  ```bash
  rusty-buddy chat --one-shot --message "Hello, assistant!"
  ```

  - **With Piped Input**
  
    ```bash
    echo "What does the program do?" | rusty-buddy chat --one-shot
    ```

## Commit Message

### Description:
Automatically generate clear and consistent commit messages from your current git diff.

### Command:

```bash
git add .
rusty-buddy commit-message
```

## Icon and Background Generation

### Description:
Create graphics based on user descriptions, utilizing state-of-the-art AI.

### Generate a Background:

- **Landscape Orientation**

  ```bash
  rusty-buddy create-background --orientation landscape --output ./backgrounds
  ```

  - **With Piped Input**
  
    ```bash
    echo "Create a sunset-themed background" | rusty-buddy create-background --orientation landscape --output ./backgrounds
    ```

- **Portrait Orientation**

  ```bash
  rusty-buddy create-background --orientation portrait --output ./backgrounds
  ```

  - **With Piped Input**
  
    ```bash
    echo "Create a cityscape background" | rusty-buddy create-background --orientation portrait --output ./backgrounds
    ```

### Generate an Icon:

- **Specify Output Sizes**

  Create an icon with multiple sizes.

  ```bash
  rusty-buddy create-icon --output ./icons --sizes 16,32,64,128,256,512
  ```

  - **With Piped Input**

    ```bash
    echo "Design a circular blue logo" | rusty-buddy create-icon --output ./icons --sizes 64,128,256
    ```

## Wish

### Description:
Fulfill development tasks by creating and modifying files and directories.

### Command:

```bash
rusty-buddy wish ./src --tools
```

## Slash Commands in Chat

Within a chat session, enrich the experience with slash commands:

- **Renew Context:**

  Refresh the chat context, clearing previous interactions and reloading specified directory files.

  ```
  /renew
  ```

- **Save Files:**

  Save code blocks from the assistant's message to a file.

  ```
  /save-files
  ```

  Options: **y** to save, **n** to skip. Quickly save all with:

  ```
  /save-files greedy
  ```

- **Save Last Answer:**

  Preserve the entire last chat response in `last_answer.txt`.

  ```
  /save-last-answer
  ```

Type the slash command within the chat interface and use `exit` to end the session, optionally saving it under a specific name.

---

By leveraging these commands, you can greatly enhance your development workflow with the capabilities of Rusty Buddy. Whether you need to generate commit messages, engage in AI-assisted chats, or create engaging visuals, this toolset provides comprehensive support.