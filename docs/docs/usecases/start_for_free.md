# Getting Started with Rusty Buddy for Free Using Ollama

Rusty Buddy is a powerful command-line tool designed to enhance your development workflow with AI-driven functionalities. This guide will walk you through the steps to set up Rusty Buddy using Ollama for free on Windows, macOS, and Linux.

## Step 1: Install Ollama
Install the Ollama service on your computer.

=== "Windows"
    Go to ollama.com and follow download instructions
=== "MacOS"
        :::bash
        brew install ollama
=== "Linux"
        ::bash
        curl -fsSL https://ollama.com/install.sh | sh

## Step 2: Download Model
I recommend using the Llama 3.2 Model.

=== "Windows"
        :::bash
        ollama pull llama3.2
=== "MacOS"
        :::bash
        ollama pull llama3.2
=== "Linux"
        :::bash
        ollama pull llama3.2

### Step 3: Install Rusty Buddy

The installation process may vary depending on your operating system. The following tabs provide step-by-step guidance for Windows, macOS, and Linux.

=== "Windows"
        :::bash
        # Open PowerShell or Command Prompt
        curl -sSL https://get.rusty-buddy.org | bash -s -- -p

    - After the installation completes, add Rusty Buddy to your PATH by modifying your profile settings accordingly.

    - Verify the installation by running:

            :::bash
            rusty-buddy --version
=== "MacOS"
        :::bash
        # Open Terminal
        curl -sSL https://get.rusty-buddy.org | bash -s -- -p

    - Ensure Rusty Buddy is accessible in your shell by updating your `.zshrc` or `.bash_profile`.

    - Confirm the installation by executing:

            :::bash
            rusty-buddy --version
=== "Linux"
        :::bash
        # Open Terminal
        curl -sSL https://get.rusty-buddy.org | bash -s -- -p

    - Ensure Rusty Buddy is accessible in your shell by updating your `.zshrc` or `.bash_profile`.

    - Confirm the installation by executing:

            :::bash
            rusty-buddy --version



### Step 2: Initialize Rusty Buddy

Once you have Rusty Buddy installed, you need to configure it for the first time. Open your terminal (Command Prompt or PowerShell for Windows) and run the following command:

```bash
rusty-buddy init
```

This command will prompt you to choose an AI backend. Make sure to select **Ollama**:

```plaintext
Choose backend to use (1 for OpenAI, 2 for Ollama): [User enters 2]
```

Next, provide the necessary configurations, such as the Ollama API URL (default is `http://localhost:11434`).

### Step 3: Start Using Rusty Buddy

With Ollama configured, you can start using Rusty Buddy to enhance your development. Below are a couple of commands to get you started:

- To start a new chat session using Ollama:
```bash
rusty-buddy chat --new
```

- To generate a commit message based on your changes:
```bash
git add .
rusty-buddy commit-message
```

### Additional Tips

- Ensure that Ollama is up and running in the background whenever you use Rusty Buddy.
- Explore additional commands and features using:
```bash
rusty-buddy --help
```

## Summary

Congratulations! You’ve successfully set up Rusty Buddy using Ollama for free on your machine. By following this guide, you can now leverage the power of AI to streamline your development workflow and enhance productivity.

---

**Happy Coding with Rusty Buddy!**