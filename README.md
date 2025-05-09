# LLM Prompt Generator

A command-line tool for generating LLM prompts with embedded file contents.
This tool makes it easy to create complex prompts that include code files from
your project, properly formatted for optimal LLM understanding.

## Features

- Expands file references in template files using `{filename}` syntax
- Formats file contents with proper markdown code blocks
- Adds file path headings for better context

## Installation

### Prerequisites

- Rust and Cargo (install from [rustup.rs](https://rustup.rs/))

### Building from source

```bash
# Clone the repository
git clone https://github.com/tmartinfr/llm-prompt-generator.git
cd llm-prompt-generator

# Install
cargo install --path .
```

## Usage

1. Create a template file with placeholders for your code files:

```
Hello LLM, I have a bug in my project.

Here's my main file:
{src/main.rs}

And here's the error I'm getting:
{error.log}

Can you help me fix this issue?
```

2. Run the tool with your template file:

```bash
llm-prompt-generator my_template.txt
```

3. The expanded output will be printed to stdout, which you can:
   - Copy directly to your LLM chat
   - Redirect to a file: `llm-prompt-generator my_template.txt > expanded_prompt.txt`
   - Pipe to another command: `llm-prompt-generator my_template.txt | pbcopy`

