---
layout: page
title: Getting Started with Rusty Buddy CLI
short_title: Getting Started
permalink: /getting-started/
---

# Getting Started with Rusty Buddy CLI

Welcome to Rusty Buddy CLI! This guide will walk you through the installation process, setup, and basic usage to get you up and running efficiently.

## Installation

Rusty Buddy CLI can be installed quickly using a script, or manually by cloning the repository and building it. Choose the method that suits your requirements.

### Quick Installation Using Script

For a quick setup on Unix-based systems (Linux/macOS), run the following command in your terminal:

{% highlight bash %}
curl -sSL https://get.rusty-buddy.org | bash
{% endhighlight %}

### Manual Installation

#### Clone and Build

1. **Clone this repository:**
{% highlight bash %}
   git clone https://github.com/hg8496/rusty-buddy.git
{% endhighlight %}

2. **Change to the project directory:**
{% highlight bash %}
   cd rusty-buddy
{% endhighlight %}

3. **Build the project:**
{% highlight bash %}
   cargo build --release
{% endhighlight %}

4. **Set up an environment file for OpenAI API key:**
{% highlight bash %}
   OPENAI_KEY=your_openai_api_key
{% endhighlight %}

#### Platform-Specific Installation

Refer to our [Download Page](download) for platform-specific installation guides for Windows, macOS, and Linux.

## Initial Setup

### Run the Init Command

To configure Rusty Buddy CLI with necessary credentials and settings, use the *init* command:

{% highlight bash %}
rusty-buddy init
{% endhighlight %}

### Configuration File

A *config.toml* file is created in the *.rusty* directory. You can customize it to tailor personas and settings as per your requirements.

## Basic Usage

Once installed, you can start using Rusty Buddy CLI to enhance your development process.

### Common Commands

- **Generate Background Images:**
{% highlight bash %}
  rusty-buddy create-background --orientation landscape --output ./backgrounds
{% endhighlight %}

- **Generate Commit Messages:**
{% highlight bash %}
  git add .
  rusty-buddy commit-message
{% endhighlight %}

- **Start a Chat Session:**
{% highlight bash %}
  rusty-buddy chat --new
{% endhighlight %}

- **Fulfill Development Wishes:**
{% highlight bash %}
  rusty-buddy wish ./src --tools
{% endhighlight %}

## Explore More

- **Documentation**: Delve into [comprehensive documentation](https://github.com/hg8496/rusty-buddy) for more features and advanced usage.
- **Subscribe**: Stay updated on our latest features and improvements by subscribing to our newsletter.

[Get Started Banner]({{ '/assets/images/getting_started4-25.png' | relative_url }}) <!-- Add a compelling image or graphic if available -->

---

Happy coding! With Rusty Buddy CLI, streamline your tasks and focus on what truly matters.