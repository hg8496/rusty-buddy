# Getting Started

Welcome to Rusty Buddy! This guide will help you quickly set up and start using Rusty Buddy's powerful features to enhance your development workflow. Follow these steps to get started.

## Initial Configuration

After installing Rusty Buddy, the first step is to configure it using the `init` command. This command streamlines the setup process, ensuring all necessary configurations and credentials are in place.

### Prerequisites

Before using Rusty Buddy, make sure you have:

- **OpenAI API Key (if using OpenAI):** Have your OpenAI API key ready for initial setup.

- **Ollama Installed (if using Ollama):** Ensure Ollama is installed and running if you choose to use it as your backend. For more information, visit [Ollama's official website](https://ollama.ai).

- **Network Access:** Required to communicate with your chosen AI backend during persona recommendation and normal usage.

### Running the Init Command

To start the configuration process, open your terminal and navigate to the directory where you've installed Rusty Buddy. Run the following command:

```bash
rusty-buddy init
```

### Execution Flow and Features

Upon running the `init` command, you will be guided through the following steps:

1. **Choose Your AI Backend**

   Rusty Buddy supports multiple AI backends. You will be prompted to select your preferred backend:

```plaintext
Choose backend to use (1 for OpenAI, 2 for Ollama): [User enters 1 or 2]
```

- **Option 1: OpenAI**

  If you choose OpenAI, you will be prompted to enter your OpenAI API key.

```plaintext
You chose OpenAI.
Please enter your OpenAI API key: [User enters key]
```

  The key is stored securely in a `.env` file in the current directory for future use.

- **Option 2: Ollama**

  If you choose Ollama, you will be prompted to enter the Ollama API URL and model name.

```plaintext
You chose Ollama.
Please enter the Ollama API URL (default: http://localhost:11434): [User enters URL or presses Enter for default]
Please enter the Ollama model (default: llama3.2): [User enters model name or presses Enter for default]
```

2. **Project Analysis and Persona Recommendation**

   Rusty Buddy will analyze your project directory to recommend the most suitable persona by scanning your files and structure.

```plaintext
Analyzing project directory...
Recommended persona: [Recommended Persona]
```

3. **Creating Configuration Files**

   The `init` command will generate the necessary configuration files:

   - A `.env` file for storing your API keys securely.
   - A `config.toml` file for your settings, including the recommended persona and model configurations.

   These files are created in the `.rusty` directory within your project.

### Choosing Your AI Backend

Depending on your selection during the `init` process, your `config.toml` will be configured for either OpenAI or Ollama.

#### Using the OpenAI Backend

If you selected the OpenAI backend, ensure that your `.env` file contains your OpenAI API key:

```plaintext
OPENAI_KEY=your_openai_api_key
```

Your `config.toml` will include configurations similar to:

```toml
default_persona = "[Recommended Persona]"

[ai]
chat_model = "openai_complex"
commit_model = "openai_fast"
wish_model = "openai_complex"

[[models]]
name = "openai_fast"
api_name = "gpt-4o-mini"
backend = "OpenAI"

[[models]]
name = "openai_complex"
api_name = "gpt-4o"
backend = "OpenAI"
```

#### Using the Ollama Backend

If you selected the Ollama backend, your `config.toml` will include the Ollama configuration:

```toml
default_persona = "[Recommended Persona]"

[ai]
chat_model = "ollama_complex"
commit_model = "ollama_complex"
wish_model = "ollama_complex"

[[models]]
name = "ollama_complex"
api_name = "llama3.2"
backend = "Ollama"
url = "http://localhost:11434"
```

### Example Usage

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

## Using Knowledge Augmentation in Chat

Rusty Buddy includes a powerful feature that allows you to add relevant knowledge to your chat context using the `--knowledge` flag.

When you use the `--knowledge` flag, the Rusty Buddy tool takes your latest user input and searches its knowledge base for any related documents or information that might help you before the assistant provides a response. This is especially useful for highly specialized topics or when dealing with project-specific queries that might require additional documentation.

### Example:

If you're working on a Rust-based project and need documentation about memory safety, you can generate knowledge and chat using:

```bash
rusty-buddy chat --persona rust --knowledge
```

This command will:
1. Use the Rust persona for targeted assistance.
2. Leverage the `knowledge` feature to search for relevant information based on your latest input.
3. Provide a more contextually aware response.

Similarly, you can use it in combination with other flags, such as `--directory` or `--model` to refine the chat experience further.

## Additional Resources

- **Installation Guide:** Refer to the [Installation](installation.md) section for detailed instructions on installing Rusty Buddy and any additional steps required for Ollama.

- **Configuration Guide:** Learn how to customize Rusty Buddy in the [Configuration Guide](configuration.md), including setting timeout durations and managing personas.

- **Commands Overview:** Explore the various [CLI Commands](commands.md) available in Rusty Buddy to maximize your productivity.

- **Working with Personas:** Learn how to customize and use different personas in the [Personas](personas.md) section.

- **Use Cases:** Check out practical examples and use cases in the [Use Cases](usecases/case_study_rust.md) section to see Rusty Buddy in action.

---

By following these simple steps, you can leverage the full functionality of Rusty Buddy and integrate it seamlessly into your software development process. Whether you're starting new chat sessions, managing commits, or generating icons, Rusty Buddy is equipped to assist you.