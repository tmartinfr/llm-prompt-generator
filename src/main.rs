use anyhow::{Context, Result, anyhow};
use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

/// A CLI tool to generate LLM prompts by expanding file references in templates
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the template file
    #[arg(help = "Path to the template file containing {filename} placeholders")]
    template_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read the template file
    let template = fs::read_to_string(&args.template_file)
        .with_context(|| format!("Failed to read template file: {:?}", args.template_file))?;

    // Extract all filenames from the template
    let filenames = extract_filenames(&template)?;

    // Check if all files are readable before proceeding
    check_files_readable(&filenames)?;

    // Find and replace all placeholders
    let expanded_template = expand_template(&template)?;

    // Output the expanded template
    println!("{}", expanded_template);

    Ok(())
}

fn extract_filenames(template: &str) -> Result<Vec<String>> {
    let re = Regex::new(r"\{([^}]+)\}")?;
    let mut filenames = Vec::new();

    for cap in re.captures_iter(template) {
        let filename = cap.get(1).unwrap().as_str().to_string();
        filenames.push(filename);
    }

    Ok(filenames)
}

fn check_files_readable(filenames: &[String]) -> Result<()> {
    let mut errors = Vec::new();

    for filename in filenames {
        if let Err(err) = fs::metadata(filename) {
            errors.push(format!("Cannot access file '{}': {}", filename, err));
        }
    }

    if !errors.is_empty() {
        return Err(anyhow!(
            "The following files are not accessible:\n{}",
            errors.join("\n")
        ));
    }

    Ok(())
}

fn expand_template(template: &str) -> Result<String> {
    let re = Regex::new(r"\{([^}]+)\}")?;
    let mut result = template.to_string();

    for cap in re.captures_iter(template) {
        let placeholder = cap.get(0).unwrap().as_str();
        let filename = cap.get(1).unwrap().as_str();

        // We've already checked that all files are readable, but let's handle errors just in case
        let file_content = match fs::read_to_string(filename) {
            Ok(content) => format!("```\n{}\n```", content),
            Err(err) => format!("[Error reading file '{}': {}]", filename, err),
        };

        result = result.replace(placeholder, &file_content);
    }

    Ok(result)
}
