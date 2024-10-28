---
layout: home
title: Welcome to Rusty Buddy CLI
short_title: Welcome
---
# Welcome to Rusty Buddy

**Empower Your Development Workflow with AI**

![Rusty Buddy CLI Logo]({{ '/assets/images/rusty-buddy-logo-128.png' | relative_url }})

Rusty Buddy is an **AI-powered command-line tool** designed to supercharge your development workflow. Whether you're seeking to generate code automatically, refine commit messages, or engage in insightful AI-assisted chats, Rusty Buddy brings state-of-the-art machine learning directly to your terminal.

---

# Rusty Buddy v1.2.0 Release Notes 🎉

We are thrilled to announce the release of Rusty Buddy v1.2.0, featuring groundbreaking improvements that elevate your development experience. The highlight of this release is the **Image-Inclusive Chat Feature**, enabling richer interactions with AI.

## 🚀 Major Features and Improvements

### Image Upload in Chat 📷
- **Visual Context Integration**: Enhance your chat sessions by uploading images alongside your textual inputs. This feature allows Rusty Buddy to analyze images and provide more insightful responses.
- **Easy to Use**: Simply use the `--image` or `-i` flag with the chat command to include an image file.

#### Example:
```bash
rusty-buddy chat --new --image ./path/to/your_image.png --persona rust
```

### Improved Documentation 📚
- A polished documentation site with mkdocs-material for a modern and visually appealing user experience.
- New examples and detailed guides on leveraging the full potential of Rusty Buddy's AI features, including image uploads.

### New CLI Commands and Enhancements 🤖
- **Enhanced Flexibility**: Support for multiple directory specifications and direct message input options simplify command usability and enhance user control.

## 🛠️ Other Improvements and Fixes
- Refined error handling for enhanced robustness across applications.
- Codebase refactorings for improved maintainability, including context management and image message handling.
- Added shell autocompletions for improved command-line interactions.

With Rusty Buddy v1.2.0, explore new dimensions of productivity and creativity by combining text and imagery in your AI interactions. We are committed to continuously improving your toolset to make development smoother, faster, and more enjoyable. We extend our gratitude to the community for its support and contributions.

Enjoy coding with Rusty Buddy! 🚀

---

## Why Choose Rusty Buddy?

**Streamline Workflows**:
> Automate mundane tasks and focus on building amazing software.

**AI-Powered Assistance**:
> Leverage advanced AI models for code generation, debugging, documentation, and more.

**Privacy First**:
> Rusty Buddy operates entirely within your local environment, ensuring no data collection and giving you complete control and peace of mind[^1].

---

## Security and Reliability

**Developed in Rust**:
> Benefit from Rust's memory safety guarantees, ensuring a secure and reliable toolset.

## Get Started Today

**[Download Now](https://www.rusty-buddy.org/download)**
> Unleash the power of AI in your development workflow and start transforming the way you code.

**[Learn More](https://docs.rusty-buddy.org)**
> Read our extensive documentation to maximize your productivity with Rusty Buddy.

**[Join Our Community](https://github.com/hg8496/rusty-buddy)**
> Contribute, share feedback, and connect with other users on GitHub.

---

![Get Started]({{ '/assets/images/getting_started4-25.png' | relative_url }}){: width="100%" }

[^1]: Applies if you are using a local Ollama installation for AI workloads.
