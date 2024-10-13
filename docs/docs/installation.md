# Installation

Rusty Buddy provides two main methods for installation: using a convenient script for a quick setup and manually cloning and building the project from the source. This section focuses on the one-step installation process using the provided script, which includes new options for customization.

## Method 1: Install Using the Installation Script

For a quick and easy installation, use the Rusty Buddy installation script. This script now includes additional features that allow for more customized installations:

    :::bash
    curl -sSL https://get.rusty-buddy.org | bash -s -- [OPTIONS]

### New Features in the Installation Script

**Options:**

- **`-p`:**  
    This option installs a Zsh plugin for Rusty Buddy, enhancing the Zsh shell experience with additional command-line integrations tailored for Rusty Buddy.

    **How to Use:**

        :::bash
        curl -sSL https://get.rusty-buddy.org | bash -s -- -p

    After installation, add `rusty-buddy` to your plugins list in the `~/.zshrc` file to activate it:

        :::bash
        plugins=(... rusty-buddy)

- **`-d <directory>`:**  
    Use this option to specify an installation directory for the Rusty Buddy binary, with the default being `$HOME/.local/bin`. Modify this to install Rusty Buddy in a different directory of your choice.

    **How to Use:**

        :::bash
        curl -sSL https://get.rusty-buddy.org | bash -s -- -d /your/custom/path

    Ensure the specified directory is included in your system's `PATH` environment variable to access Rusty Buddy from any terminal.

**Script Details:**

- The script automatically determines your operating system and architecture to download the appropriate Rusty Buddy binary.
- If the specified installation directory (`-d <directory>`) is not in your `PATH`, the script provides a guide on adding it to your shell configuration.
- With the `-p` option, the Zsh plugin is installed into the `~/.oh-my-zsh/custom/plugins/rusty-buddy` directory.

These enhancements make installing Rusty Buddy more customizable and compatible with different development environments.

## Method 2: Clone and Build

If you prefer to have more control over the installation or need to modify the source, you can clone the repository and build the project yourself. Here are the steps:

1. **Clone the Repository:**

    Clone the Rusty Buddy GitHub repository to your local environment:

        :::bash
        git clone https://github.com/hg8496/rusty-buddy.git

2. **Navigate to the Project Directory:**

    Change your current directory to the project root:

        :::bash
        cd rusty-buddy

3. **Build the Project:**

    Use Cargo, Rust's package manager, to build Rusty Buddy. Ensure you are building in release mode for optimal performance:

        :::bash
        cargo build --release

4. **Set Up the Environment:**

    Follow the information in the [Getting Started Guide](getting_started.md) for easy setup.

**Prerequisites:**

- Ensure that Rust and Cargo are installed on your system. You can install them via [rustup](https://rustup.rs/).
- Network access may be required for both installation methods, particularly for downloading dependencies or connecting with the AI backends.

---

By following these instructions, you will set up Rusty Buddy and harness its capabilities for your development workflows. Choose the installation method that aligns with your needs and system configuration.

---

**Happy Coding with Rusty Buddy!**
