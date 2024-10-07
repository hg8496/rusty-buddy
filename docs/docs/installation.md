# Installation

Rusty Buddy provides two main methods for installation: using a convenient script for quick installation and manually cloning and building the project from the source. Follow the method that suits your environment.

## Method 1: Install Using Script

For a quick and easy installation, you can use the provided script to download and install Rusty Buddy:

```bash
curl -sSL https://get.rusty-buddy.org | bash
```

**Note:**
- Ensure that you have the necessary permissions to run installation scripts on your system.
- This method will download the Rusty Buddy binaries to your machine.

## Method 2: Clone and Build

If you prefer to have more control over the installation or need to modify the source, you can clone the repository and build the project yourself. Here are the steps:

1. **Clone the Repository:**

   Begin by cloning the Rusty Buddy GitHub repository to your local environment:

```bash
git clone https://github.com/hg8496/rusty-buddy.git
```

2. **Navigate to the Project Directory:**

   Change your current directory to the project root:

```bash
cd rusty-buddy
```

3. **Build the Project:**

   Use Cargo, Rust's package manager, to build Rusty Buddy. Ensure you are building in release mode for optimal performance:

```bash
cargo build --release
```

4. **Set Up the Environment:**

For an easy setup follow the information in the [getting started guide](getting_started.md)

**Prerequisites:**
- Ensure that Rust and Cargo are installed on your system. You can install them via [rustup](https://rustup.rs/).
- Network access may be required for both installation methods, particularly for downloading dependencies or connecting with the AI backends.

## Additional Requirements for Ollama

To use the Ollama backend with Rusty Buddy, you need to install and configure the Ollama service. The following steps will guide you through the process.

### Step 1: Install Ollama

Ollama needs to be installed separately from Rusty Buddy. Follow these steps:

- **For macOS:**

  Install Ollama using Homebrew:

```bash
brew install ollama
```

- **For other platforms:**

  Visit the [Official Ollama Installation Guide](https://ollama.ai/download) for instructions tailored to your operating system.

### Step 2: Load an Ollama Model

After installing Ollama, you need to download and load the desired AI model.

For example, to download the `llama2` model, run:

```bash
ollama pull llama2
```

You can view available models on the [Ollama Models Page](https://ollama.ai/library).

### Step 3: Verify Ollama Installation

Ensure that the Ollama service is running and accessible. By default, Ollama runs on `http://localhost:11434`.

You can test it by running:

```bash
ollama serve
```

### Step 4: Configure Rusty Buddy for Ollama

When you run the `rusty-buddy init` command, select Ollama as your AI backend. Follow the prompts to enter the Ollama API URL and model name.

Your `config.toml` should be configured to use Ollama:

```toml
default_persona = "[Recommended Persona]"

[ai]
chat_model = "ollama_complex"
commit_model = "ollama_complex"
wish_model = "ollama_complex"

[[models]]
name = "ollama_complex"
api_name = "llama2"
backend = "Ollama"
url = "http://localhost:11434"
```

**Note:**
- Replace `llama2` with the name of the model you have downloaded.
- Ensure that the `url` matches the address where Ollama is running.

## Final Steps

With Rusty Buddy installed and configured, you're ready to start using it. Refer to the [Getting Started](getting_started.md) guide to initialize the application and begin integrating Rusty Buddy into your development workflow.

---

By following these instructions, you will set up Rusty Buddy and harness its capabilities for your development workflows. Choose the installation method that aligns with your needs and system configuration.

---

**Happy Coding with Rusty Buddy!**