# Working with Personas

Rusty Buddy comes equipped with customizable personas to tailor AI interactions according to your specific development needs. Personas help maintain context, tone, and expertise specific to different programming languages or roles.

## Built-in Personas

Rusty Buddy includes several built-in personas, each tailored for specific technologies and roles. Here's a list of some available personas:

| Persona Type                        | ID                            |
|-------------------------------------|-------------------------------|
| Rust Developer                      | rust                          |
| Swift Developer                     | swift                         |
| Java Developer                      | java                          |
| TypeScript Developer                | typescript                    |
| Embedded C Specialist               | embedded_c_specialist         |
| Python Developer                    | python                        |
| JavaScript Developer                | javascript                    |
| C++ Developer                       | cplusplus                     |
| C# Developer                        | csharp                        |
| DevOps Engineer                     | devops_engineer               |
| Data Scientist                      | data_scientist                |
| UX/UI Designer                      | ux_ui_designer                |

To see a full list of available personas, refer to the official Rusty Buddy documentation or the config file descriptions.

## Configuring Personas

You can configure and customize personas in the `config.toml` file located within your project's `.rusty` directory. The configuration file allows you to adjust prompts, specify file types, and set default personas.

### Configuration File Structure

Here's an example structure of the `config.toml` file with persona configurations:

```toml
# Default persona to use if none is specified
default_persona = "rust"

# Model configurations for different functionalities
[ai]
hat_model = "openai_complex"
commit_model = "openai_fast"
wish_model = "openai_complex"

#Define various models
[[models]]
name = "openai_fast"
api_name = "gpt-4o-mini"
backend = "OpenAI"

[[models]]
name = "openai_complex"
api_name = "gpt-4o-2024-08-06"
backend = "OpenAI"

# Define various personas
[[personas]]
name = "rust"
chat_prompt = "You are an experienced Rust developer assisting with Rust programming queries."
file_types = ["rs", "md", "toml"]

[[personas]]
name = "python"
chat_prompt = "You are an experienced Python developer ready to tackle Python-related problems."
file_types = ["py", "md"]
```

### Steps to Configure a Custom Persona

1. **Locate the Configuration File:**

   Find the `config.toml` file in the `.rusty` directory within your project.

2. **Adding or Modifying Personas:**

   To add or modify a persona, use the following syntax in the configuration file:

   ```toml
   [[personas]]
   name = "persona_name"
   chat_prompt = "Description of the persona's role and expertise."
   file_types = ["file_extension_1", "file_extension_2"]
   ```

   - **name:** The identifier for the persona.
   - **chat_prompt:** A brief description or role that dictates how the AI should interact.
   - **file_types:** Types of files that should be included or emphasized for this persona.

3. **Set a Default Persona:**

   Specify a default persona that Rusty Buddy will default to if none is explicitly requested.

   ```toml
   default_persona = "your_default_persona"
   ```

4. **Restart Rusty Buddy:**

   After making changes, restart Rusty Buddy to apply your configuration updates.

## Using Personas in Commands

You can specify a persona when running chat-related commands:

```bash
rusty-buddy chat --persona python
```

This command initializes a chat session using the specified persona, affecting the tone and focus of interactions with the AI assistant.

## Best Practices

- **Contextual Relevance:** Align personas with the primary languages and roles within your project to ensure relevance.
- **Clarity in Prompts:** Provide clear and concise chat prompts to define expectations and interactions.
- **Regular Updates:** Keep persona configurations updated as project roles and technologies evolve.

By effectively configuring and using personas, you can significantly enhance AI interactions, making them more relevant and aligned with user expectations in various development environments.