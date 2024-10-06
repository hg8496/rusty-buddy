# Getting Started

Welcome to Rusty Buddy! This guide will help you quickly set up and start using Rusty Buddy's powerful features to enhance your development workflow. Follow these steps to get started.

## Initial Configuration

After installing Rusty Buddy, the first step is to configure it using the `init` command. This command streamlines the setup process, ensuring all necessary configurations and credentials are in place.

### Running the Init Command

To start the configuration process, open your terminal, and navigate to the directory where you've installed Rusty Buddy. Run the following command:

```bash
rusty-buddy init
```

### Execution Flow and Features

1. **OpenAI API Key Prompt:**
   - If the OpenAI API key is not found in your environment, Rusty Buddy will prompt you to enter it.
   - The key is stored securely in a `.env` file in the current directory for future use.

```text
OpenAI key not found in the environment.
Please enter your OpenAI API key: [User enters key]
```

2. **Persona Recommendation:**
   - Rusty Buddy performs a scan of your project directory, analyzing the files and structure.
   - It uses this information to recommend a suitable persona by communicating with the OpenAI API.
  
```text
Analyzing project directory...
Recommended persona: [Recommended Persona]
```

3. **Creating Configuration Files:**
   - A default `config.toml` file is generated in the `.rusty` directory.
   - This file includes the recommended persona and sets default models for chat and commit message generation.

## Choosing Your AI Provider

After configuring your environment, you can select between different AI backends, including OpenAI and Ollama, depending on your needs or preferences.

### Using the Ollama Backend

To utilize Ollama, ensure your configuration in the `config.toml` specifies Ollama in the desired models:
```toml
[ai]
chat_model = "ollama_32"
commit_model = "ollama_32"
wish_model = "ollama_32"

[[models]]
name = "ollama_32"
api_name = "llama3.2"
backend = "Ollama"
url = "http://localhost:11434"

```

## Example Usage

Once your setup is complete, you can start using Rusty Buddy right away. Here are a few common scenarios:

- **Start a New Chat Session:**

  Initiate a new conversation with the AI assistant to discuss ideas or troubleshoot issues.

```bash
rusty-buddy chat --new
```

- **Generate a Commit Message:**

  Automatically generate descriptive commit messages from your current git diff.

```bash
git add .
rusty-buddy commit-message
```

### Prerequisites

Before using Rusty Buddy, make sure you have:

- **OpenAI API Key:** Have your OpenAI API key ready for initial setup.
- **Network Access:** Required to communicate with OpenAI servers during persona recommendation and normal usage.

By following these simple steps, you can leverage the full functionality of Rusty Buddy and integrate it seamlessly into your software development process. Whether you're starting new chat sessions, managing commits, or generating icons, Rusty Buddy is equipped to assist you.