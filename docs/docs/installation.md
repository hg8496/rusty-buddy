# Installation

Rusty Buddy provides two main methods for installation: using a convenient script for quick installation and manually cloning and building the project from the source. Follow the method that suits your environment:

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

   Begin by cloning the Rusty Buddy GitHub repository to your local environment using the following command:

   ```bash
   git clone https://github.com/hg8496/rusty-buddy.git
   ```

2. **Navigate to the Project Directory:**

   Change your current directory to the project root:

   ```bash
   cd rusty-buddy
   ```

3. **Build the Project:**

   Use Cargo, Rust's package manager, to build Rusty Buddy. Ensure you are building in release mode for the best performance:

   ```bash
   cargo build --release
   ```

**Prerequisites:**
- Ensure that Rust and Cargo are installed on your system. You can install them via [rustup](https://rustup.rs/).
- Network access may be required for both installation methods, particularly for downloading dependencies or connecting with the OpenAI API.

By following these instructions, you will be able to set up Rusty Buddy and harness its capability for your development workflows. Choose the installation method that aligns with your needs and system configuration.

## Additional Requirements for Ollama

To use the Ollama feature in Rusty Buddy, you need to install and configure the Ollama service. This section explains any additional dependencies or steps required for Ollama.

### Step 1: Install Ollama

Ensure that the Ollama service is installed and running on your machine. You can follow the installation guide on the [official Ollama documentation](https://ollama.com).

### Step 2: Configure Firewall and Ports

Make sure your network allows communication through the port that Ollama uses (default is 11434).