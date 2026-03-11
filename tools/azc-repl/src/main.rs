//! AZC REPL
//!
//! Interactive Read-Eval-Print Loop for AZC.

use anyhow::Result;
use clap::Parser;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::validate::{
    MatchingBracketValidator, ValidationContext, ValidationResult, Validator,
};
use rustyline::{Config, EditMode, Editor};
use std::collections::HashMap;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "azc-repl")]
#[command(about = "AZC REPL", long_about = None)]
struct Cli {
    /// Load file on startup
    #[arg(short, long)]
    load: Option<String>,
}

struct Repl {
    variables: HashMap<String, String>,
    functions: HashMap<String, String>,
    history: Vec<String>,
    compiler_path: String,
}

impl Repl {
    fn new() -> Self {
        Repl {
            variables: HashMap::new(),
            functions: HashMap::new(),
            history: Vec::new(),
            compiler_path: "./compiler/target/release/azc".to_string(),
        }
    }

    fn evaluate(&mut self, input: &str) -> Result<String> {
        let input = input.trim();

        // Check for commands
        if input.starts_with(':') {
            return self.handle_command(input);
        }

        // Try to evaluate as expression
        let code = if input.starts_with("let ")
            || input.starts_with("def ")
            || input.starts_with("struct ")
            || input.starts_with("enum ")
        {
            input.to_string()
        } else {
            // Wrap expression in puts
            format!("puts ({})", input)
        };

        // Create temporary file
        let temp_file = "/tmp/azc_repl.azc";
        std::fs::write(temp_file, &code)?;

        // Compile
        let output = Command::new(&self.compiler_path).arg(temp_file).output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let c_code = String::from_utf8(output.stdout)?;
        let c_file = "/tmp/azc_repl.c";
        std::fs::write(c_file, &c_code)?;

        // Compile C to executable
        let exe_file = "/tmp/azc_repl";
        let compile_output = Command::new("gcc")
            .arg("-o")
            .arg(exe_file)
            .arg(c_file)
            .output()?;

        if !compile_output.status.success() {
            return Err(anyhow::anyhow!("C compilation failed"));
        }

        // Run executable
        let run_output = Command::new(exe_file).output()?;

        Ok(String::from_utf8_lossy(&run_output.stdout)
            .trim()
            .to_string())
    }

    fn handle_command(&mut self, input: &str) -> Result<String> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            return Ok("".to_string());
        }

        match parts[0] {
            ":help" => Ok(format!(
                "AZC REPL v0.1.0\n\n\
                    Commands:\n\
                    :help          Show this help\n\
                    :quit          Exit REPL\n\
                    :clear         Clear screen\n\
                    :load <file>   Load a file\n\
                    :type <expr>   Show type of expression\n\
                    :vars          List all variables\n\
                    :funcs         List all functions\n\
                    :history       Show command history\n\
                    :reset         Reset environment\n\
                    \n\
                    Examples:\n\
                    let x = 5\n\
                    x + 10\n\
                    def add(a, b) a + b end\n\
                    add(1, 2)"
            )),
            ":quit" | ":q" | ":exit" => {
                std::process::exit(0);
            }
            ":clear" => {
                print!("\x1B[2J\x1B[1;1H");
                Ok("".to_string())
            }
            ":vars" => {
                if self.variables.is_empty() {
                    Ok("No variables defined".to_string())
                } else {
                    let vars: Vec<String> = self
                        .variables
                        .iter()
                        .map(|(k, v)| format!("{} = {}", k, v))
                        .collect();
                    Ok(vars.join("\n"))
                }
            }
            ":funcs" => {
                if self.functions.is_empty() {
                    Ok("No functions defined".to_string())
                } else {
                    let funcs: Vec<String> = self
                        .functions
                        .iter()
                        .map(|(k, v)| format!("def {} ... end", k))
                        .collect();
                    Ok(funcs.join("\n"))
                }
            }
            ":history" => {
                if self.history.is_empty() {
                    Ok("No history".to_string())
                } else {
                    Ok(self.history.join("\n"))
                }
            }
            ":reset" => {
                self.variables.clear();
                self.functions.clear();
                Ok("Environment reset".to_string())
            }
            ":load" => {
                if parts.len() < 2 {
                    return Ok("Usage: :load <file>".to_string());
                }
                let filename = parts[1];
                let content = std::fs::read_to_string(filename)?;
                let result = self.evaluate(&content)?;
                Ok(format!(
                    "Loaded {} lines from {}",
                    content.lines().count(),
                    filename
                ))
            }
            ":type" => {
                if parts.len() < 2 {
                    return Ok("Usage: :type <expr>".to_string());
                }
                // Simplified type inference
                let expr = parts[1..].join(" ");
                if expr.parse::<i64>().is_ok() {
                    Ok("Int".to_string())
                } else if expr.parse::<f64>().is_ok() {
                    Ok("Float".to_string())
                } else if expr == "true" || expr == "false" {
                    Ok("Bool".to_string())
                } else if expr.starts_with('"') {
                    Ok("String".to_string())
                } else {
                    Ok("Unknown".to_string())
                }
            }
            _ => Ok(format!("Unknown command: {}", parts[0])),
        }
    }

    fn add_to_history(&mut self, input: &str) {
        if !input.trim().is_empty() {
            self.history.push(input.to_string());
        }
    }
}

struct InputValidator {
    brackets: MatchingBracketValidator,
}

impl Validator for InputValidator {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        self.brackets.validate(ctx)
    }
}

fn main() -> Result<()> {
    let _cli = Cli::parse();

    println!("{}", "AZC REPL v0.1.0".bold().green());
    println!("Type :help for commands");
    println!();

    let mut repl = Repl::new();

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(rustyline::CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    let helper = InputValidator {
        brackets: MatchingBracketValidator::new(),
    };

    let mut rl: Editor<InputValidator, DefaultHistory> = Editor::with_config(config)?;
    rl.set_helper(Some(helper));

    loop {
        let readline = rl.readline(&format!("{} ", "azc>".green().bold()));

        match readline {
            Ok(line) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line)?;
                repl.add_to_history(line);

                match repl.evaluate(line) {
                    Ok(result) => {
                        if !result.is_empty() {
                            println!("{} {}", "=>".blue().bold(), result);
                        }
                    }
                    Err(e) => {
                        eprintln!("{} {}", "Error:".red(), e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
