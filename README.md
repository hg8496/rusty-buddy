# Rusty Buddy

## Empower Your Development Workflow with AI

![Rusty Buddy Logo](https://www.rusty-buddy.org/assets/images/rusty-buddy-logo-128.png)

Rusty Buddy is an **AI-powered command-line tool** designed to supercharge your development workflow. Whether you're 
seeking to generate code automatically, refine commit messages, or engage in insightful AI-assisted chats, 
Rusty Buddy brings state-of-the-art machine learning directly to your terminal.

---

## Rusty Buddy v1.3.0 Release Notes 🚀

We are excited to announce **Rusty Buddy v1.3.0**! This release delivers **robust improvements, smarter onboarding, and enhanced flexibility for power users and teams**. Here are the highlights:

---

### 🎯 Major Improvements

#### 1. **Smarter Persona Recommendation & Interactive Onboarding**
- **Recursive Project Analysis:**  
  Rusty Buddy now **recursively scans your full project directory (including all subfolders)** during `rusty-buddy init`. This enables automatic and highly-relevant persona recommendations based on *your* project's files and structure!
- **Manual Persona Selection:**  
  Prefer full control? Use
  ```bash
  rusty-buddy init --choose-persona
  ```
  to skip the scan and pick any available persona from a menu.

#### 2. **Commit Message Generation With Context**
- **Developer Intent Support:**  
  The `commit-message` command now lets you provide an *optional developer intent* for more accurate, context-aware commit summaries:
  ```bash
  rusty-buddy commit-message --developer-intent "Implement feature X"
  ```
  This is especially useful for complex or multi-faceted changes.

#### 3. **CLI Usability Enhancements**
- **Command-Line Argument Parsing:**  
  The command-line parser is improved: all `init` and `commit-message` options are now properly handled and documented in both `--help` and completions.
- **Expanded Troubleshooting Section:**  
  Documentation now covers edge cases for environment, configuration file location, missing API keys, and more.

---

## Why Choose Rusty Buddy?

- **Streamline Workflows**  
  Automate mundane tasks and focus on building amazing software.

- **AI-Powered Assistance**  
  Leverage advanced AI models for code generation, debugging, documentation, and more.

- **Context-Aware Interaction**  
  Rusty Buddy adapts to your project's context, providing relevant suggestions and insights.

- **Customizable Personas**  
  Tailor AI interactions with a variety of personas specialized in different domains.

---

## Key Features

- **Intuitive Chat Interface**  
  Engage in natural language conversations with the AI assistant to brainstorm ideas, solve problems, or get code assistance.

- **Slash Command Auto-Completion**  
  Boost your efficiency with intelligent command suggestions and auto-completion within the chat interface.

- **Commit Message Generator**  
  Generate clear and consistent commit messages automatically from your git diffs, following best practices.

- **Icon and Background Image Generation**  
  Create stunning visuals effortlessly with AI-generated graphics based on your descriptions.

- **Tool Integration**  
  Automate repetitive tasks by allowing the AI to interact with your files and directories, enhancing your development process.

- **Shell Completion Support**  
  Improve your command-line experience with shell completion scripts for Bash, Zsh, Fish, and more.

---

## Security and Privacy

- **Developed in Rust**  
  Benefit from Rust's memory safety guarantees, ensuring a secure and reliable toolset.

- **Privacy First**  
  Rusty Buddy operates entirely within your local environment[^1], with **no data collection**, giving you complete control and peace of mind.

[^1]: If you are using a local Ollama installation for AI workloads.

---

## Installation

You can set up Rusty Buddy using either the quick script method or by building it from source.

### Method 1: Install Using Script

For a hassle-free setup, execute the following script:

```bash
curl -sSL https://get.rusty-buddy.org | bash
```

**Note:** Ensure you have the necessary permissions to run installation scripts on your system.

### Method 2: Clone and Build

1. **Clone the Repository**

   ```bash
   git clone https://github.com/hg8496/rusty-buddy.git
   ```

2. **Navigate to the Project Directory**

   ```bash
   cd rusty-buddy
   ```

3. **Build the Project**

   ```bash
   cargo build --release
   ```

---

## Getting Started

After installation, perform initial configurations using the `init` command:

```bash
rusty-buddy init
```

The setup process will:

1. **Choose Your AI Backend**

   - **OpenAI**: Enter your OpenAI API key.
   - **Ollama**: Enter the Ollama API URL and model name.

     ```plaintext
     Choose backend to use (1 for OpenAI, 2 for Ollama): [User enters 1 or 2]
     ```

2. **Persona Recommendation**

    - Rusty Buddy *recursively scans your project directory*, including all subfolders, to build a directory listing.
      Based on the detected files and structure, it recommends the persona that best matches your project type.

      ```plaintext
      Analyzing project directory...
      Recommended persona: [Persona]
      ```
    **Tip:** Prefer to handpick your persona?
    Run `rusty-buddy init --choose-persona` and you'll pick from a menu. Otherwise, Rusty Buddy will auto-select based on a directory scan.

3. **Create Configuration Files**

   - Generates `.env` and `config.toml` files in the `.rusty` directory.

---

## Example Usage

- **Start a New Chat Session**

  ```bash
  rusty-buddy chat --new
  ```

- **Generate a Commit Message**

  ```bash
  git add .
  rusty-buddy commit-message
  ```

- **Create an Icon**

  ```bash
  rusty-buddy create-icon --output ./icons --sizes 64,128,256
  ```

- **Use a Specific Persona**

  ```bash
  rusty-buddy chat --persona rust
  ```

- **Fulfill a Wish**

  ```bash
  rusty-buddy wish ./src --tools
  ```

---

## Documentation

- **[Installation Guide](https://docs.rusty-buddy.org/installation)**
- **[Getting Started](https://docs.rusty-buddy.org/getting_started)**
- **[Commands Overview](https://docs.rusty-buddy.org/commands)**
- **[Working with Personas](https://docs.rusty-buddy.org/personas)**
- **[Use Cases](https://docs.rusty-buddy.org/usecases/case_study_rust)**

---

## Join Our Community

- **[GitHub Repository](https://github.com/hg8496/rusty-buddy)**
- **[Official Documentation](https://docs.rusty-buddy.org)**
- **[Contact Support](mailto:hg8496@cstolz.de)**

---

**Happy Coding with Rusty Buddy!**

---

![Quickstart with Rusty Buddy](https://docs.rusty-buddy.org/quickstart.gif)

---

# License

Rusty Buddy is released under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

# Contact

For further queries or support, feel free to reach out at [hg8496@cstolz.de](mailto:hg8496@cstolz.de).

---

**Embrace the future of development with Rusty Buddy—your AI-powered assistant!**
