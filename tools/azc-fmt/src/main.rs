//! AZC Code Formatter
//!
//! Formats AZC code according to style guidelines.

use anyhow::Result;
use clap::Parser;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "azc-fmt")]
#[command(about = "Format AZC code", long_about = None)]
struct Cli {
    /// Files to format
    files: Vec<PathBuf>,

    /// Check formatting without making changes
    #[arg(short, long)]
    check: bool,

    /// Print diff
    #[arg(short, long)]
    diff: bool,

    /// Config file path
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct Config {
    max_width: Option<usize>,
    tab_size: Option<usize>,
    use_tabs: Option<bool>,
    indent_style: Option<String>,
    newline_style: Option<String>,
    trailing_comma: Option<bool>,
    semicolons: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_width: Some(100),
            tab_size: Some(4),
            use_tabs: Some(false),
            indent_style: Some("space".to_string()),
            newline_style: Some("unix".to_string()),
            trailing_comma: Some(true),
            semicolons: Some(false),
        }
    }
}

struct Formatter {
    config: Config,
    indent_level: usize,
}

impl Formatter {
    fn new(config: Config) -> Self {
        Formatter {
            config,
            indent_level: 0,
        }
    }

    fn format(&self, source: &str) -> Result<String> {
        let mut result = String::new();
        let mut in_multiline_string = false;
        let mut in_comment = false;
        let lines: Vec<&str> = source.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Skip empty lines
            if trimmed.is_empty() {
                result.push('\n');
                continue;
            }

            // Preserve comments
            if trimmed.starts_with('#') && !trimmed.starts_with("#{") {
                result.push_str(trimmed);
                result.push('\n');
                continue;
            }

            // Handle multiline strings
            if trimmed.contains("\"\"\"") {
                in_multiline_string = !in_multiline_string;
                result.push_str(trimmed);
                result.push('\n');
                continue;
            }

            if in_multiline_string {
                result.push_str(line);
                result.push('\n');
                continue;
            }

            // Decrease indent for end/else/catch
            if trimmed.starts_with("end")
                || trimmed.starts_with("else")
                || trimmed.starts_with("elsif")
                || trimmed.starts_with("catch")
                || trimmed.starts_with("when")
            {
                self.indent_level = self.indent_level.saturating_sub(1);
            }

            // Format the line
            let formatted = self.format_line(trimmed)?;

            // Add indentation
            let indent = self.config.tab_size.unwrap_or(4) * self.indent_level;
            let indent_str = if self.config.use_tabs.unwrap_or(false) {
                "\t".repeat(indent / 4)
            } else {
                " ".repeat(indent)
            };

            result.push_str(&indent_str);
            result.push_str(&formatted);
            result.push('\n');

            // Increase indent after def/if/while/class/struct/enum/impl
            if trimmed.starts_with("def ")
                || trimmed.starts_with("if ")
                || trimmed.starts_with("while ")
                || trimmed.starts_with("class ")
                || trimmed.starts_with("struct ")
                || trimmed.starts_with("enum ")
                || trimmed.starts_with("impl ")
                || trimmed.starts_with("for ")
                || trimmed.starts_with("match ")
                || trimmed.starts_with("try")
                || trimmed == "else"
                || trimmed.starts_with("elsif ")
            {
                self.indent_level += 1;
            }
        }

        Ok(result)
    }

    fn format_line(&self, line: &str) -> Result<String> {
        let mut result = line.to_string();

        // Normalize spaces around operators
        let ops = [
            "+", "-", "*", "/", "%", "==", "!=", "<=", ">=", "<", ">", "and", "or", "=",
        ];

        for op in &ops {
            // Add spaces around operators
            let pattern = format!(r"(\S){}(\S)", regex::escape(op));
            let re = Regex::new(&pattern)?;
            result = re
                .replace_all(&result, &format!("$1 {} $2", op))
                .to_string();

            // Remove extra spaces
            let pattern = format!(r"\s+{}\s+", regex::escape(op));
            let re = Regex::new(&pattern)?;
            result = re.replace_all(&result, &format!(" {} ", op)).to_string();
        }

        // Remove spaces after opening brackets
        let re = Regex::new(r"\(\s+")?;
        result = re.replace_all(&result, "(").to_string();

        let re = Regex::new(r"\[\s+")?;
        result = re.replace_all(&result, "[").to_string();

        let re = Regex::new(r"\{\s+")?;
        result = re.replace_all(&result, "{").to_string();

        // Remove spaces before closing brackets
        let re = Regex::new(r"\s+\)")?;
        result = re.replace_all(&result, ")").to_string();

        let re = Regex::new(r"\s+\]")?;
        result = re.replace_all(&result, "]").to_string();

        let re = Regex::new(r"\s+\}")?;
        result = re.replace_all(&result, "}").to_string();

        // Add space after commas
        let re = Regex::new(r",(\S)")?;
        result = re.replace_all(&result, ", $1").to_string();

        // Normalize colons in type annotations
        let re = Regex::new(r":(\S)")?;
        result = re.replace_all(&result, ": $1").to_string();

        // Normalize hash rockets
        let re = Regex::new(r"=>")?;
        result = re.replace_all(&result, "=>").to_string();

        // Remove trailing whitespace
        let trimmed = result.trim_end();

        // Remove semicolons if configured
        if !self.config.semicolons.unwrap_or(false) {
            let re = Regex::new(r";\s*$")?;
            Ok(re.replace(trimmed, "").to_string())
        } else {
            Ok(trimmed.to_string())
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::default();

    if cli.files.is_empty() {
        // Format from stdin
        let input = std::io::read_to_string(std::io::stdin())?;
        let formatter = Formatter::new(config);
        let output = formatter.format(&input)?;
        print!("{}", output);
        return Ok(());
    }

    let mut changed = 0;
    let mut unchanged = 0;
    let mut errors = 0;

    for file in &cli.files {
        if !file.exists() {
            eprintln!("Error: File not found: {}", file.display());
            errors += 1;
            continue;
        }

        let content = fs::read_to_string(file)?;
        let formatter = Formatter::new(config.clone());
        let formatted = formatter.format(&content)?;

        if cli.check {
            if content != formatted {
                println!("Would format: {}", file.display());
                changed += 1;

                if cli.diff {
                    for (old, new) in content.lines().zip(formatted.lines()) {
                        if old != new {
                            println!("- {}", old);
                            println!("+ {}", new);
                        }
                    }
                }
            } else {
                unchanged += 1;
            }
        } else {
            if content != formatted {
                fs::write(file, formatted)?;
                println!("Formatted: {}", file.display());
                changed += 1;
            } else {
                unchanged += 1;
            }
        }
    }

    if cli.check {
        println!();
        println!(
            "Summary: {} changed, {} unchanged, {} errors",
            changed.to_string().red(),
            unchanged.to_string().green(),
            errors.to_string().yellow()
        );

        if changed > 0 {
            std::process::exit(1);
        }
    }

    Ok(())
}
