Welcome to Rusty Buddy, your AI-powered development companion designed to streamline coding workflows and enhance productivity. Whether you’re seeking to generate code automatically, refine commit messages, or engage in insightful AI-assisted chats, Rusty Buddy brings state-of-the-art machine learning to your development toolkit.

![Rusty Buddy Logo](https://www.rusty-buddy.org/assets/images/rusty-buddy-logo-128.png)

## Unleash Your Development Potential

Rusty Buddy is more than just a command-line tool; it’s a suite of utilities tailored to modern developers:

- **AI-Enhanced Coding**: Leverage AI to automate mundane tasks, generate intelligent code snippets, and manage your projects efficiently.
- **Context-Aware Interactions**: Elevate your coding experience with an AI that adapts based on your project’s context and needs.
- **Versatile Personas**: Customize the AI's interaction style to fit various programming languages and roles, ensuring precise and context-specific guidance.

[Get Started Now](https://docs.rusty-buddy.org/getting_started) and transform your coding experience with Rusty Buddy.

## Security

At Rusty Buddy, your security and privacy are our top priorities:

- **Built on Rust**: Rusty Buddy is developed using Rust, known for its focus on memory safety and concurrent development practices. This reduces the risk of vulnerabilities, ensuring a robust and secure toolset.
  
- **Privacy First**: We do not collect any of your usage data. Rusty Buddy operates entirely within your local environment[^1], giving you full control and peace of mind regarding your privacy.

## Installation

You can set up Rusty Buddy using either the quick script method or by building it from the source:

### Method 1: Install Using Script

For a hassle-free setup, execute the following script:

```bash
curl -sSL https://get.rusty-buddy.org | bash
```

This downloads and installs the necessary binaries. Ensure you have the appropriate permissions.

### Method 2: Clone and Build

1. Clone the repository:
   ```bash
   git clone https://github.com/hg8496/rusty-buddy.git
   ```
2. Navigate to the project directory:
   ```bash
   cd rusty-buddy
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. Set up an environment file to include your OpenAI API key:
   ```plaintext
   OPENAI_KEY=your_openai_api_key
   ```

## Initial Setup

After installation, perform initial configurations using the `init` command:

```bash
rusty-buddy init
```

This setup process will prompt you for your OpenAI API key, recommend a persona based on your project's characteristics, and generate necessary configuration files.

## Configuration and Customization

Rusty Buddy supports extensive customization via a `config.toml` file located in the `.rusty` directory. This file allows you to tailor the AI models, set default personas, and define language-specific configurations.

### Customizing Personas

Modify the `config.toml` to adjust or add personas to fit your project's needs:

```toml
[[personas]]
name = "python"
chat_prompt = "You are an experienced Python developer."
file_types = ["py", "md"]
```

Set a default persona for projects:

```toml
default_persona = "rust"
```

For detailed configuration options, please visit our [Documentation](https://docs.rusty-buddy.org/personas).

## Shell Completion

Enhance your command-line experience with shell completion scripts for Bash, Zsh, Fish, or PowerShell:

- For Bash:
  ```bash
  rusty-buddy --completion=bash >> ~/.bashrc
  ```
- For Zsh:
  ```zsh
  rusty-buddy --completion=zsh >> ~/.zshrc
  ```

Reload your shell afterwards to activate it.

## Contributing

We welcome contributions! To collaborate, fork the repository, make improvements, and submit a pull request for review.

## License

Rusty Buddy is released under the MIT License. Review the [LICENSE](LICENSE) file for more details.

## Contact

For further queries or support, feel free to reach out at [hg8496@cstolz.de](mailto:hg8496@cstolz.de).

Embrace the future of development with Rusty Buddy—your AI-powered assistant!

[^1]: Your data will be sent to OpenAI API to use AI features.