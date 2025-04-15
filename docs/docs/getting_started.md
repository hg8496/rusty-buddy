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
**How Persona Selection Works:**
During `rusty-buddy init`, Rusty Buddy **recursively scans your current directory and all subdirectories** to gather a complete listing of files and folders. It uses this structure and file types to
intelligently determine the nature of your project—such as Rust, Python, Web, Embedded, etc.—and then recommends the most suitable persona.
This ensures the AI interactions you receive are tailored precisely to your project’s ecosystem.

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

### Motivating Knowledge Enhancement

To truly maximize the potential of Rusty Buddy, consider building a rich knowledge database. By adding documents such as coding guides, design patterns, and architecture documentation, you empower the AI to provide more contextually relevant and insightful assistance.

**Why Add Documents?**

- **Enhanced Contextual Understanding:** Equip the AI with domain-specific context, allowing it to offer more precise and tailored guidance.

- **Streamline Onboarding:** New team members can access comprehensive resources directly through the AI's knowledge base, reducing onboarding time and resources.

- **Facilitate Code Reviews and Development:** Let Rusty Buddy cross-reference architecture guides and coding standards in real-time for enriched code reviews and development advice.

### How to Add Documents:

- **Coding Guides and Documentation:**

```bash
rusty-buddy knowledge add --file path/to/coding_guide.md
```

- **Design and Architecture Documents:**

```bash
rusty-buddy knowledge add --dir path/to/architecture_docs
```

Initiating this habit enables you and your team to leverage the full intelligence of Rusty Buddy, turning it into an invaluable partner in every stage of your development workflow.

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

By following these simple steps and enriching the knowledge database, you can leverage the full functionality of Rusty Buddy, integrating it seamlessly into your software development process and achieving new productivity heights. Whether you're starting new chat sessions, managing commits, or generating icons, Rusty Buddy is equipped to assist you.

---

**Happy Coding with Rusty Buddy!**