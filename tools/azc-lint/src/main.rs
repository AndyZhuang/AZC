//! AZC Linter
//!
//! Static analysis and linting for AZC code.

use anyhow::Result;
use clap::Parser;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "azc-lint")]
#[command(about = "Lint AZC code", long_about = None)]
struct Cli {
    /// Files to lint
    files: Vec<PathBuf>,

    /// Automatically fix issues
    #[arg(short, long)]
    fix: bool,

    /// Output format
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Fail on warnings
    #[arg(long)]
    deny_warnings: bool,

    /// Only show errors
    #[arg(long)]
    errors_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Lint {
    code: String,
    level: LintLevel,
    title: String,
    description: String,
    help: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum LintLevel {
    Error,
    Warning,
    Info,
    Style,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LintResult {
    file: String,
    line: usize,
    column: usize,
    lint: Lint,
    source: String,
}

struct Linter {
    results: Vec<LintResult>,
    fix_enabled: bool,
}

impl Linter {
    fn new(fix_enabled: bool) -> Self {
        Linter {
            results: Vec::new(),
            fix_enabled,
        }
    }

    fn lint(&mut self, source: &str, filename: &str) -> Result<()> {
        let lines: Vec<&str> = source.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line_num = line_num + 1; // 1-indexed

            // Skip comments
            let trimmed = line.trim();
            if trimmed.starts_with('#') && !trimmed.starts_with("#{") {
                continue;
            }

            // Check for various lint conditions
            self.check_unused_variables(trimmed, line_num, filename);
            self.check_long_lines(line, line_num, filename);
            self.check_trailing_whitespace(line, line_num, filename);
            self.check_naming_conventions(trimmed, line_num, filename);
            self.check_comparison_to_bool(trimmed, line_num, filename);
            self.check_deprecated_syntax(trimmed, line_num, filename);
            self.check_empty_blocks(trimmed, line_num, filename, &lines);
            self.check_complex_expressions(trimmed, line_num, filename);
            self.check_missing_docs(trimmed, line_num, filename);
            self.check_potential_integer_overflow(trimmed, line_num, filename);
        }

        Ok(())
    }

    fn check_unused_variables(&mut self, line: &str, line_num: usize, filename: &str) {
        // Simplified check - would need proper AST analysis
        if line.starts_with("let ") && line.contains('=') {
            // Check if variable is used in subsequent lines (simplified)
            // This is a placeholder for proper analysis
        }
    }

    fn check_long_lines(&mut self, line: &str, line_num: usize, filename: &str) {
        if line.len() > 100 {
            self.results.push(LintResult {
                file: filename.to_string(),
                line: line_num,
                column: 100,
                lint: Lint {
                    code: "S001".to_string(),
                    level: LintLevel::Style,
                    title: "Line too long".to_string(),
                    description: format!("Line is {} characters (max 100)", line.len()),
                    help: Some("Consider breaking this line into multiple lines".to_string()),
                },
                source: line.to_string(),
            });
        }
    }

    fn check_trailing_whitespace(&mut self, line: &str, line_num: usize, filename: &str) {
        if line != line.trim_end() && !line.is_empty() {
            self.results.push(LintResult {
                file: filename.to_string(),
                line: line_num,
                column: line.len(),
                lint: Lint {
                    code: "S002".to_string(),
                    level: LintLevel::Style,
                    title: "Trailing whitespace".to_string(),
                    description: "Line has trailing whitespace".to_string(),
                    help: Some("Remove trailing whitespace".to_string()),
                },
                source: line.to_string(),
            });
        }
    }

    fn check_naming_conventions(&mut self, line: &str, line_num: usize, filename: &str) {
        // Check function names (should be snake_case)
        if line.starts_with("def ") {
            if let Some(name) = line.strip_prefix("def ") {
                let name = name.split('(').next().unwrap_or("").trim();
                if name.contains(char::is_uppercase) {
                    self.results.push(LintResult {
                        file: filename.to_string(),
                        line: line_num,
                        column: 5,
                        lint: Lint {
                            code: "S003".to_string(),
                            level: LintLevel::Style,
                            title: "Function name should be snake_case".to_string(),
                            description: format!("Function '{}' should use snake_case", name),
                            help: Some(format!("Consider: {}", to_snake_case(name))),
                        },
                        source: line.to_string(),
                    });
                }
            }
        }

        // Check constants (should be SCREAMING_SNAKE_CASE)
        if line.starts_with("let ") && line.contains('=') {
            // Simplified - would need proper constant detection
        }
    }

    fn check_comparison_to_bool(&mut self, line: &str, line_num: usize, filename: &str) {
        // Check for == true or == false
        if line.contains("== true") || line.contains("== false") {
            self.results.push(LintResult {
                file: filename.to_string(),
                line: line_num,
                column: 1,
                lint: Lint {
                    code: "S004".to_string(),
                    level: LintLevel::Warning,
                    title: "Comparison to boolean literal".to_string(),
                    description: "Comparing to true/false is unnecessary".to_string(),
                    help: Some("Use the value directly or apply 'not'".to_string()),
                },
                source: line.to_string(),
            });
        }
    }

    fn check_deprecated_syntax(&mut self, line: &str, line_num: usize, filename: &str) {
        // Check for deprecated patterns
        if line.contains("return;") {
            self.results.push(LintResult {
                file: filename.to_string(),
                line: line_num,
                column: 1,
                lint: Lint {
                    code: "S005".to_string(),
                    level: LintLevel::Warning,
                    title: "Unnecessary return".to_string(),
                    description: "Empty return is unnecessary at end of function".to_string(),
                    help: Some("Remove the return statement".to_string()),
                },
                source: line.to_string(),
            });
        }
    }

    fn check_empty_blocks(&mut self, line: &str, line_num: usize, filename: &str, lines: &[&str]) {
        // Check for empty if/while/def blocks
        if line.starts_with("if ") || line.starts_with("while ") || line.starts_with("def ") {
            if line_num < lines.len() {
                let next_line = lines[line_num].trim();
                if next_line == "end" {
                    self.results.push(LintResult {
                        file: filename.to_string(),
                        line: line_num,
                        column: 1,
                        lint: Lint {
                            code: "S006".to_string(),
                            level: LintLevel::Warning,
                            title: "Empty block".to_string(),
                            description: "Block has no content".to_string(),
                            help: Some("Add code to the block or remove it".to_string()),
                        },
                        source: line.to_string(),
                    });
                }
            }
        }
    }

    fn check_complex_expressions(&mut self, line: &str, line_num: usize, filename: &str) {
        // Count operators
        let operator_count = line.matches(|c: char| "+-*/%=&|<>!".contains(c)).count();

        if operator_count > 5 {
            self.results.push(LintResult {
                file: filename.to_string(),
                line: line_num,
                column: 1,
                lint: Lint {
                    code: "S007".to_string(),
                    level: LintLevel::Info,
                    title: "Complex expression".to_string(),
                    description: format!("Expression has {} operators", operator_count),
                    help: Some("Consider breaking into smaller expressions".to_string()),
                },
                source: line.to_string(),
            });
        }
    }

    fn check_missing_docs(&mut self, line: &str, line_num: usize, filename: &str) {
        // Check for undocumented public functions
        if line.starts_with("def ") && !line.contains("private") {
            // Simplified - would need to check previous line for comments
        }
    }

    fn check_potential_integer_overflow(&mut self, line: &str, line_num: usize, filename: &str) {
        // Check for potential overflow in arithmetic
        if line.contains('+') || line.contains('*') {
            // Simplified - would need type information
        }
    }

    fn print_results(&self, format: &str, errors_only: bool) {
        match format {
            "json" => {
                let filtered: Vec<_> = self
                    .results
                    .iter()
                    .filter(|r| !errors_only || matches!(r.lint.level, LintLevel::Error))
                    .collect();
                println!("{}", serde_json::to_string(&filtered).unwrap());
            }
            _ => {
                for result in &self.results {
                    if errors_only && !matches!(result.lint.level, LintLevel::Error) {
                        continue;
                    }

                    let level_str = match result.lint.level {
                        LintLevel::Error => "error".red(),
                        LintLevel::Warning => "warning".yellow(),
                        LintLevel::Info => "info".blue(),
                        LintLevel::Style => "style".cyan(),
                    };

                    println!(
                        "{}: {}[{}] {}",
                        format!("{}:{}:{}", result.file, result.line, result.column).bold(),
                        level_str,
                        result.lint.code,
                        result.lint.title
                    );

                    println!("  {}", result.lint.description);

                    if let Some(help) = &result.lint.help {
                        println!("  {} {}", "help:".green(), help);
                    }

                    println!();
                }
            }
        }
    }

    fn summary(&self) -> (usize, usize, usize, usize) {
        let mut errors = 0;
        let mut warnings = 0;
        let mut info = 0;
        let mut style = 0;

        for result in &self.results {
            match result.lint.level {
                LintLevel::Error => errors += 1,
                LintLevel::Warning => warnings += 1,
                LintLevel::Info => info += 1,
                LintLevel::Style => style += 1,
            }
        }

        (errors, warnings, info, style)
    }
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.files.is_empty() {
        eprintln!("Error: No files specified");
        std::process::exit(1);
    }

    let mut linter = Linter::new(cli.fix);
    let mut processed = 0;
    let mut errors_files = 0;

    for file in &cli.files {
        if !file.exists() {
            eprintln!("Error: File not found: {}", file.display());
            errors_files += 1;
            continue;
        }

        if file.extension().map_or(true, |ext| ext != "azc") {
            continue;
        }

        let content = fs::read_to_string(file)?;
        linter.lint(&content, &file.display().to_string())?;
        processed += 1;
    }

    linter.print_results(&cli.format, cli.errors_only);

    let (errors, warnings, info, style) = linter.summary();

    println!("{}", "─".repeat(60));
    println!(
        "Summary: {} error(s), {} warning(s), {} info, {} style",
        errors.to_string().red(),
        warnings.to_string().yellow(),
        info.to_string().blue(),
        style.to_string().cyan()
    );
    println!("{} file(s) processed", processed);

    if errors > 0 || (cli.deny_warnings && warnings > 0) {
        std::process::exit(1);
    }

    Ok(())
}
