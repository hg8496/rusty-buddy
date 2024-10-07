# Configuration Guide

Rusty Buddy provides a flexible configuration system using the `config.toml` file located in your project's `.rusty` directory. This guide outlines the configuration options available to tailor Rusty Buddy to your preferences.

---

## Configuration File Location

The `config.toml` file is located within the `.rusty` directory in your project's root folder:

```bash
your_project/
├── .rusty/
│   └── config.toml
```

If the `.rusty` directory or `config.toml` does not exist, you can generate it by running:

```bash
rusty-buddy init
```

---

## Configurable Options

### **1. Default Persona**

Set the default persona that Rusty Buddy uses if none is specified during a chat session.

```toml
default_persona = "rust"
```

### **2. AI Models Configuration**

Configure the AI models for different functionalities:

```toml
[ai]
chat_model = "openai_complex"
commit_model = "openai_fast"
wish_model = "openai_complex"
chat_timeout_secs = 30  # Timeout duration in seconds
```

- **`chat_model`**: Model used for chat sessions.
- **`commit_model`**: Model used for generating commit messages.
- **`wish_model`**: Model used for the `wish` command.
- **`chat_timeout_secs`**: **(New)** Timeout duration for AI responses in seconds.

**Example:**

```toml
[ai]
chat_model = "openai_complex"
commit_model = "openai_fast"
wish_model = "openai_complex"
chat_timeout_secs = 90  # Set timeout to 15 minutes
```

### **3. Models Definition**

Define the AI models and their backends:

```toml
[[models]]
name = "openai_fast"
api_name = "gpt-4o-mini"
backend = "OpenAI"

[[models]]
name = "openai_complex"
api_name = "gpt-4o-2024-08-06"
backend = "OpenAI"
```

### **4. Personas Configuration**

Customize or add new personas to enhance your AI interactions:

```toml
[[personas]]
name = "rust"
chat_prompt = "As a seasoned Rust developer..."
file_types = ["rs", "toml", "md", "yml"]
```

---

## Configuring the Timeout Duration

**New Feature:** You can now configure the timeout duration for AI responses using the `chat_timeout_secs` parameter in your `config.toml` file. This setting ensures that all chat AI interactions timeout consistently based on your preference.

### **Setting the Timeout**

1. Open your `config.toml` file located in `.rusty/config.toml`.

2. Under the `[ai]` section, add or modify the `chat_timeout_secs` parameter:

```toml
[ai]
chat_model = "your_chat_model"
commit_model = "your_commit_model"
wish_model = "your_wish_model"
chat_timeout_secs = 60  # Timeout set to 1 minute
```

3. Save the file. The new timeout duration will be applied to all subsequent AI interactions.

### **Understanding Timeout Behavior**

- The `chat_timeout_secs` parameter defines the maximum duration (in seconds) Rusty Buddy will wait for a response from the AI backend before timing out.
- Adjusting this value can help manage long-running requests or prevent delays in your workflow.

**Example:**

- **Shorter Timeout (e.g., `chat_timeout_secs = 30`):** Useful for quick interactions where responses are expected promptly.
- **Longer Timeout (e.g., `chat_timeout_secs = 90`):** Allows the AI more time to process complex or resource-intensive requests.

---

## Advanced Configuration

Feel free to explore and adjust other settings in the `config.toml` file to better suit your development needs. Always ensure that the syntax is correct to prevent configuration errors.

---

**Note:** Any changes to the `config.toml` file require restarting Rusty Buddy to take effect.

---

## Troubleshooting

If you encounter issues:

- **Invalid Configuration Syntax:** Ensure your `config.toml` follows proper TOML syntax.
- **Timeout Errors:** If requests are timing out too soon or taking too long, adjust the `chat_timeout_secs` value accordingly.
- **Contact Support:** If problems persist, reach out via [Contact Support](mailto:hg8496@cstolz.de).

---

**Happy Coding with Rusty Buddy!**