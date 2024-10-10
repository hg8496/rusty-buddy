# Working with Personas

Rusty Buddy comes equipped with customizable personas to tailor AI interactions according to your specific development needs. Personas help maintain context, tone, and expertise specific to different programming languages or roles.

## Built-in Personas

Rusty Buddy includes a comprehensive set of built-in personas, each tailored for specific technologies and roles. Below is the complete list of available personas extracted from the source code, along with brief descriptions:

| Persona Type                          | ID                              | Description                                                                                                                                                        |
|---------------------------------------|---------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Rust Developer                        | rust                            | A seasoned Rust developer guiding you through coding challenges, architectural designs, and best practices in Rust programming.                                    |
| Swift Developer                       | swift                           | An experienced Swift developer assisting with feature implementation, resolving Swift-specific issues, and optimizing code for better performance.                 |
| Java Developer                        | java                            | A knowledgeable Java developer supporting you with foundational and advanced aspects of Java programming, including design patterns and JVM optimizations.        |
| TypeScript Developer                  | typescript                      | An experienced TypeScript developer eager to assist with type systems, async programming, and integrating TypeScript with existing JavaScript projects.           |
| Python Developer                      | python                          | A proficient Python developer helping with script optimization, framework selection, and solving Python-specific issues across data science and web development.   |
| Embedded C Specialist                 | embedded_c_specialist           | An expert in developing firmware and low-level software for microcontrollers, optimizing C code for performance in resource-constrained environments.             |
| Yocto C++ Embedded Specialist         | yocto_cpp_embedded_specialist   | Assists with building custom Linux distributions for embedded systems using the Yocto Project and provides insights into C++ development and cross-compiling.      |
| JavaScript Developer                  | javascript                      | A skilled JavaScript developer ready to assist from DOM manipulation to modern front-end frameworks, enhancing functionality and performance in your projects.     |
| C++ Developer                         | cplusplus                       | An experienced C++ developer offering guidance on systems programming, memory management, and effective use of the STL, aiming to optimize code for performance.  |
| C# Developer                          | csharp                          | A C# developer adept at guiding you through .NET development, LINQ queries, and asynchronous programming, ensuring robust application architecture.               |
| PHP Developer                         | php                             | An experienced PHP developer ready to assist with web application development, server-side scripting, and database integration with systems like MySQL.            |
| Ruby Developer                        | ruby                            | A Ruby developer capable of helping with Ruby on Rails applications, gem management, and optimizing code for concise and clean syntax.                            |
| Go Developer                          | golang                          | A Go developer supporting your work with highly concurrent and efficient applications, guiding package usage and performance tuning.                             |
| Kotlin Developer                      | kotlin                          | Assists in Kotlin development for Android apps or JVM-based environments, focusing on clean code and leveraging Kotlin's features.                              |
| R Developer                           | r                               | An R developer helping with statistical computing, data visualization, and leveraging R's extensive package ecosystem for analytical tasks.                     |
| Scala Developer                       | scala                           | A Scala developer supporting functional programming, concurrent processing, and leveraging the power of the JVM through Scala.                                   |
| Shell Scripting Expert                | shell                           | Skilled in shell scripting, assists with script automation, task scheduling, and system management using scripts.                                               |
| Perl Developer                        | perl                            | A Perl developer helping with text processing, automation, and leveraging Perl's adaptability for diverse tasks.                                                |
| Dart Developer                        | dart                            | A Dart developer focused on assisting with Flutter app development, providing insights into reactive programming and efficient UI building.                      |
| Objective-C Developer                 | objective-c                     | Versed in Objective-C, helps with maintaining older iOS/macOS applications and bridging code with Swift for modernization.                                      |
| LaTeX Book Setter                     | latex_book_setter               | Specializes in formatting scholarly articles, books, and papers with precision using LaTeX, assisting with document layout and typesetting equations.            |
| Poet                                  | poet                            | A poet eager to inspire and enhance your creative expression through words, supporting your poetic endeavors with rhythm and meter.                             |
| Technical Writer                      | technical_writer                | An experienced technical writer helping you craft clear, concise, and user-friendly documentation, ensuring content is informative and accessible.              |
| Novelist                              | novelist                        | A novelist ready to help you develop compelling narratives and characters, providing insights into storytelling techniques for your literary works.             |
| Screenwriter                          | screenwriter                    | A screenwriter providing guidance on crafting scripts that captivate audiences, from writing dialogue to structuring your screenplay.                            |
| Journalist                            | journalist                      | A journalist assisting with news writing, reporting techniques, and editorial processes, ensuring your articles are accurate and engaging.                       |
| Content Writer                        | content_writer                  | A content writer helping you craft engaging and SEO-friendly content for blogs, websites, and marketing materials.                                              |
| UX/UI Designer                        | ux_ui_designer                  | A UX/UI Designer assisting with creating engaging and user-friendly interfaces, focusing on usability and accessibility enhancements.                           |
| Data Scientist                        | data_scientist                  | A Data Scientist assisting with data analysis, statistical modeling, and leveraging machine learning techniques for actionable insights.                        |
| Cybersecurity Analyst                 | cybersecurity_analyst           | Provides guidance on identifying vulnerabilities and securing systems against potential threats, including ethical hacking practices and encryption protocols. |
| SEO Specialist                        | seo_specialist                  | An SEO Specialist dedicated to improving your website's visibility through search engine optimization, including keyword analysis and content optimization.     |
| Social Media Manager                  | social_media_manager            | Helps create and manage social media content that resonates with your audience, focusing on content scheduling and engagement strategies.                       |
| Project Manager                       | project_manager                 | A Project Manager assisting with planning, resource allocation, and timeline management to ensure successful project delivery.                                  |
| Game Developer                        | game_developer                  | Provides insights into game design, mechanics, and storytelling to help you create engaging interactive experiences.                                           |
| AI/ML Engineer                        | ai_ml_engineer                  | An AI/ML Engineer providing support in building AI models and systems using machine learning techniques, exploring neural networks and NLP.                   |
| Digital Marketing Strategist          | digital_marketing_strategist    | Assists in crafting strategies to reach your audience effectively and analyze campaign performance for better decision-making.                                 |
| DevOps Engineer                       | devops_engineer                 | Well-versed in automating deployments, CI/CD, infrastructure monitoring, and management.                                                                       |
| Ansible Expert                        | ansible_expert                  | Offers assistance in automating software provisioning, configuration management, and application deployment with Ansible.                                     |

## Viewing Available Personas

You can list all available personas using the following command:

```bash
rusty-buddy --list-personas
```

This will display all the persona IDs that you can use with Rusty Buddy.

## Configuring Personas

You can configure and customize personas in the `config.toml` file located within your project's `.rusty` directory. The configuration file allows you to adjust prompts, specify file types, and set default personas.

### Configuration File Structure

Here's an example structure of the `config.toml` file with persona configurations:

```toml
# Default persona to use if none is specified
default_persona = "rust"

# AI model configurations
[ai]
chat_model = "openai_complex"
commit_model = "openai_fast"
wish_model = "openai_complex"

# Define AI models
[[models]]
name = "openai_fast"
api_name = "gpt-4o-mini"
backend = "OpenAI"

[[models]]
name = "openai_complex"
api_name = "gpt-4o-2024-08-06"
backend = "OpenAI"

# Define personas
[[personas]]
name = "rust"
chat_prompt = '''As a seasoned Rust developer, I am here to guide you through any coding 
challenges, architectural designs, and best practices in Rust programming. Feel free to 
ask about memory safety, concurrency patterns, or help with debugging. Let's work together 
to improve code efficiency and performance.
'''
file_types = ["rs", "toml", "md", "yml"]

[[personas]]
name = "python"
chat_prompt = "As a proficient Python developer, I can help you with script optimization, framework selection, and solving Python-specific issues. Feel free to explore aspects of data science, web development, or simplify cross-platform scripting."
file_types = ["py", "ipynb", "md"]

# Add additional persona configurations as needed
```

### Steps to Configure a Custom Persona

1. **Locate the Configuration File**

   Find the `config.toml` file in the `.rusty` directory within your project.

2. **Adding or Modifying Personas**

   To add or modify a persona, use the following syntax in the configuration file:

```toml
[[personas]]
name = "persona_name"
chat_prompt = "Description of the persona's role and expertise."
file_types = ["file_extension_1", "file_extension_2"]
```

   - **name:** The identifier for the persona.
   - **chat_prompt:** A detailed description that guides the AI on how to interact.
   - **file_types:** File extensions that are relevant for this persona's context.

3. **Set a Default Persona**

   Specify a default persona that Rusty Buddy will use if none is explicitly requested.

```toml
default_persona = "your_default_persona"
```

4. **Restart Rusty Buddy**

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
- **File Types:** Ensure that the `file_types` array includes all relevant file extensions to provide appropriate context for the persona.

By effectively configuring and using personas, you can significantly enhance AI interactions, making them more relevant and aligned with user expectations in various development environments.

---

**Note:** Remember that the personas are meant to guide the AI in providing the most helpful responses tailored to your needs. Feel free to experiment with custom personas to find the best fit for your projects.