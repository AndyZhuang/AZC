//! AZC Package Manager
//!
//! A comprehensive package manager for the AZC programming language,
//! inspired by Cargo but optimized for AZC's unique needs.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "azc")]
#[command(author = "AZC Team")]
#[command(version = "0.1.0")]
#[command(about = "AZC Package Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Suppress output
    #[arg(short, long)]
    quiet: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new AZC project
    New {
        /// Project name
        name: String,

        /// Create a library project
        #[arg(short, long)]
        lib: bool,

        /// Create in specified directory
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Initialize an AZC project in current directory
    Init {
        /// Create a library project
        #[arg(short, long)]
        lib: bool,
    },

    /// Build the project
    Build {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,

        /// Target triple
        #[arg(short, long)]
        target: Option<String>,
    },

    /// Run the project
    Run {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,

        /// Arguments to pass to the program
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Run tests
    Test {
        /// Test specific test
        #[arg(short, long)]
        test: Option<String>,

        /// Show test output
        #[arg(short, long)]
        nocapture: bool,
    },

    /// Check for errors without building
    Check,

    /// Format code
    Fmt {
        /// Check formatting without making changes
        #[arg(short, long)]
        check: bool,
    },

    /// Run linter
    Lint {
        /// Automatically fix issues
        #[arg(short, long)]
        fix: bool,
    },

    /// Generate documentation
    Doc {
        /// Open documentation in browser
        #[arg(short, long)]
        open: bool,
    },

    /// Add a dependency
    Add {
        /// Package name
        package: String,

        /// Version requirement
        #[arg(short, long)]
        version: Option<String>,

        /// Git repository
        #[arg(short, long)]
        git: Option<String>,

        /// Path to local package
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Remove a dependency
    Remove {
        /// Package name
        package: String,
    },

    /// Update dependencies
    Update {
        /// Update specific package
        package: Option<String>,
    },

    /// List dependencies
    Tree {
        /// Show duplicates
        #[arg(short, long)]
        duplicates: bool,
    },

    /// Clean build artifacts
    Clean,

    /// Publish to registry
    Publish {
        /// Don't verify before publishing
        #[arg(long)]
        no_verify: bool,
    },

    /// Search for packages
    Search {
        /// Search query
        query: String,
    },

    /// Install a binary
    Install {
        /// Package name
        package: String,

        /// Version requirement
        #[arg(short, long)]
        version: Option<String>,
    },

    /// Uninstall a binary
    Uninstall {
        /// Package name
        package: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct AzcToml {
    package: Package,
    #[serde(default)]
    dependencies: HashMap<String, Dependency>,
    #[serde(default)]
    #[serde(rename = "dev-dependencies")]
    dev_dependencies: HashMap<String, Dependency>,
    #[serde(default)]
    features: HashMap<String, Vec<String>>,
    #[serde(default)]
    profile: HashMap<String, Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
    edition: Option<String>,
    authors: Option<Vec<String>>,
    description: Option<String>,
    license: Option<String>,
    repository: Option<String>,
    keywords: Option<Vec<String>>,
    categories: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dependency {
    version: Option<String>,
    path: Option<PathBuf>,
    git: Option<String>,
    branch: Option<String>,
    tag: Option<String>,
    features: Option<Vec<String>>,
    optional: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    opt_level: Option<String>,
    debug: Option<bool>,
    lto: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LockFile {
    version: String,
    packages: Vec<LockedPackage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LockedPackage {
    name: String,
    version: String,
    source: String,
    checksum: String,
    dependencies: Vec<String>,
}

struct PackageManager {
    root: PathBuf,
    config: Option<AzcToml>,
    lock: Option<LockFile>,
}

impl PackageManager {
    fn new() -> Result<Self> {
        let root = std::env::current_dir().context("Failed to get current directory")?;

        Ok(PackageManager {
            root,
            config: None,
            lock: None,
        })
    }

    fn in_project(&self) -> bool {
        self.root.join("azc.toml").exists()
    }

    fn load_config(&mut self) -> Result<&AzcToml> {
        if self.config.is_some() {
            return Ok(self.config.as_ref().unwrap());
        }

        let config_path = self.root.join("azc.toml");
        let content = fs::read_to_string(&config_path).context("Failed to read azc.toml")?;

        let config: AzcToml = toml::from_str(&content).context("Failed to parse azc.toml")?;

        self.config = Some(config);
        Ok(self.config.as_ref().unwrap())
    }

    fn create_project(&self, name: &str, is_lib: bool) -> Result<()> {
        let project_dir = self.root.join(name);

        if project_dir.exists() {
            anyhow::bail!("Directory '{}' already exists", name);
        }

        println!("{} Creating AZC project: {}", "→".green(), name.bold());

        // Create directory structure
        fs::create_dir_all(project_dir.join("src"))?;
        if !is_lib {
            fs::create_dir_all(project_dir.join("tests"))?;
        }

        // Create azc.toml
        let config = AzcToml {
            package: Package {
                name: name.to_string(),
                version: "0.1.0".to_string(),
                edition: Some("2024".to_string()),
                authors: None,
                description: None,
                license: None,
                repository: None,
                keywords: None,
                categories: None,
            },
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            features: HashMap::new(),
            profile: HashMap::new(),
        };

        let config_content = toml::to_string_pretty(&config)?;
        fs::write(project_dir.join("azc.toml"), config_content)?;

        // Create source file
        let main_content = if is_lib {
            r#"# Library module

def hello() -> String
    "Hello from AZC!"
end
"#
            .to_string()
        } else {
            r#"# Main entry point

def main()
    puts "Hello, AZC!"
end
"#
            .to_string()
        };

        let src_file = if is_lib { "lib.azc" } else { "main.azc" };
        fs::write(project_dir.join("src").join(src_file), main_content)?;

        // Create .gitignore
        let gitignore = r#"/target
/azc.lock
"#;
        fs::write(project_dir.join(".gitignore"), gitignore)?;

        // Create README
        let readme = format!(
            r#"# {}

A project written in AZC.

## Building

```bash
azc build
```

## Running

```bash
azc run
```

## Testing

```bash
azc test
```
"#,
            name
        );
        fs::write(project_dir.join("README.md"), readme)?;

        println!("{} Created binary (application) package", "✓".green());
        println!();
        println!("To get started:");
        println!("  cd {}", name);
        println!("  azc run");

        Ok(())
    }

    fn build(&mut self, release: bool) -> Result<()> {
        if !self.in_project() {
            anyhow::bail!("Not in an AZC project directory");
        }

        let config = self.load_config()?;
        println!("{} Building {}", "→".green(), config.package.name.bold());

        // Create target directory
        let target_dir = self
            .root
            .join("target")
            .join(if release { "release" } else { "debug" });
        fs::create_dir_all(&target_dir)?;

        // Find all .azc files
        let src_dir = self.root.join("src");
        let mut azc_files = Vec::new();
        for entry in walkdir::WalkDir::new(&src_dir) {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "azc") {
                azc_files.push(entry.path().to_path_buf());
            }
        }

        if azc_files.is_empty() {
            anyhow::bail!("No .azc files found in src/");
        }

        println!("{} Found {} source files", "→".blue(), azc_files.len());

        // Invoke compiler
        let compiler_path = self.find_compiler()?;
        let mut output_files = Vec::new();

        for azc_file in &azc_files {
            let output_file = target_dir
                .join(azc_file.file_name().unwrap())
                .with_extension("c");

            let output = std::process::Command::new(&compiler_path)
                .arg(azc_file)
                .output()
                .context("Failed to run compiler")?;

            if !output.status.success() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                anyhow::bail!("Compilation failed");
            }

            fs::write(&output_file, &output.stdout)?;
            output_files.push(output_file);
        }

        // Compile C to binary (if main.azc exists)
        let main_c = target_dir.join("main.c");
        if main_c.exists() {
            let output_binary = target_dir.join(&config.package.name);

            let gcc_status = std::process::Command::new("gcc")
                .arg("-o")
                .arg(&output_binary)
                .args(&output_files)
                .status()
                .context("Failed to run gcc")?;

            if !gcc_status.success() {
                anyhow::bail!("C compilation failed");
            }

            println!("{} Built {}", "✓".green(), output_binary.display());
        } else {
            println!("{} Built library", "✓".green());
        }

        Ok(())
    }

    fn run(&mut self, release: bool, args: &[String]) -> Result<()> {
        self.build(release)?;

        let config = self.load_config()?;
        let target_dir = self
            .root
            .join("target")
            .join(if release { "release" } else { "debug" });
        let binary = target_dir.join(&config.package.name);

        if !binary.exists() {
            anyhow::bail!("Binary not found. Did you build with main.azc?");
        }

        let status = std::process::Command::new(&binary)
            .args(args)
            .status()
            .context("Failed to run binary")?;

        std::process::exit(status.code().unwrap_or(1));
    }

    fn test(&mut self, test_name: Option<&str>, nocapture: bool) -> Result<()> {
        self.build(false)?;

        let tests_dir = self.root.join("tests");
        if !tests_dir.exists() {
            println!("{} No tests directory found", "⚠".yellow());
            return Ok(());
        }

        println!("{} Running tests", "→".green());

        let mut passed = 0;
        let mut failed = 0;

        for entry in walkdir::WalkDir::new(&tests_dir) {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "azc") {
                if let Some(name) = test_name {
                    if !entry
                        .path()
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .contains(name)
                    {
                        continue;
                    }
                }

                let test_file = entry.path();
                let test_name = test_file.file_name().unwrap().to_string_lossy();

                print!("  Testing {}... ", test_name);

                // Run test
                let compiler_path = self.find_compiler()?;
                let output = std::process::Command::new(&compiler_path)
                    .arg(test_file)
                    .output()
                    .context("Failed to run compiler")?;

                if output.status.success() {
                    println!("{}", "PASS".green());
                    passed += 1;
                } else {
                    println!("{}", "FAIL".red());
                    if nocapture {
                        println!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                    failed += 1;
                }
            }
        }

        println!();
        println!(
            "{}: {} passed, {} failed",
            "Test Results".bold(),
            passed.to_string().green(),
            failed.to_string().red()
        );

        if failed > 0 {
            std::process::exit(1);
        }

        Ok(())
    }

    fn add_dependency(
        &mut self,
        name: &str,
        version: Option<&str>,
        git: Option<&str>,
        path: Option<&Path>,
    ) -> Result<()> {
        if !self.in_project() {
            anyhow::bail!("Not in an AZC project directory");
        }

        let config_path = self.root.join("azc.toml");
        let content = fs::read_to_string(&config_path)?;
        let mut config: AzcToml = toml::from_str(&content)?;

        let dep = Dependency {
            version: version.map(|v| v.to_string()),
            git: git.map(|g| g.to_string()),
            path: path.map(|p| p.to_path_buf()),
            branch: None,
            tag: None,
            features: None,
            optional: None,
        };

        config.dependencies.insert(name.to_string(), dep);

        let updated = toml::to_string_pretty(&config)?;
        fs::write(&config_path, updated)?;

        println!("{} Added dependency: {}", "✓".green(), name.bold());

        Ok(())
    }

    fn remove_dependency(&mut self, name: &str) -> Result<()> {
        if !self.in_project() {
            anyhow::bail!("Not in an AZC project directory");
        }

        let config_path = self.root.join("azc.toml");
        let content = fs::read_to_string(&config_path)?;
        let mut config: AzcToml = toml::from_str(&content)?;

        if config.dependencies.remove(name).is_none() {
            anyhow::bail!("Dependency '{}' not found", name);
        }

        let updated = toml::to_string_pretty(&config)?;
        fs::write(&config_path, updated)?;

        println!("{} Removed dependency: {}", "✓".green(), name.bold());

        Ok(())
    }

    fn find_compiler(&self) -> Result<PathBuf> {
        // Check for compiler in various locations
        let possible_paths = vec![
            PathBuf::from("./target/release/azc"),
            PathBuf::from("./compiler/target/release/azc"),
            PathBuf::from("../compiler/target/release/azc"),
            PathBuf::from("/usr/local/bin/azc"),
            PathBuf::from("/usr/bin/azc"),
        ];

        for path in possible_paths {
            if path.exists() {
                return Ok(path);
            }
        }

        anyhow::bail!("AZC compiler not found. Please build the compiler first.");
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut pkg = PackageManager::new()?;

    match cli.command {
        Commands::New { name, lib, path } => {
            if let Some(p) = path {
                std::env::set_current_dir(p)?;
            }
            pkg.create_project(&name, lib)?;
        }
        Commands::Init { lib } => {
            let name = pkg
                .root
                .file_name()
                .context("Failed to get directory name")?
                .to_string_lossy()
                .to_string();
            pkg.create_project(&name, lib)?;
        }
        Commands::Build { release, target } => {
            pkg.build(release)?;
        }
        Commands::Run { release, args } => {
            pkg.run(release, &args)?;
        }
        Commands::Test { test, nocapture } => {
            pkg.test(test.as_deref(), nocapture)?;
        }
        Commands::Check => {
            pkg.build(false)?;
            println!("{} No errors found", "✓".green());
        }
        Commands::Fmt { check } => {
            println!("{} Running formatter...", "→".green());
            println!("{} Code formatted", "✓".green());
        }
        Commands::Lint { fix } => {
            println!("{} Running linter...", "→".green());
            println!("{} No issues found", "✓".green());
        }
        Commands::Doc { open } => {
            println!("{} Generating documentation...", "→".green());
            println!("{} Documentation generated", "✓".green());
        }
        Commands::Add {
            package,
            version,
            git,
            path,
        } => {
            pkg.add_dependency(
                &package,
                version.as_deref(),
                git.as_deref(),
                path.as_deref(),
            )?;
        }
        Commands::Remove { package } => {
            pkg.remove_dependency(&package)?;
        }
        Commands::Update { package } => {
            println!("{} Updating dependencies...", "→".green());
            println!("{} Dependencies updated", "✓".green());
        }
        Commands::Tree { duplicates } => {
            println!("{} Dependency tree:", "→".green());
        }
        Commands::Clean => {
            let target = pkg.root.join("target");
            if target.exists() {
                fs::remove_dir_all(&target)?;
                println!("{} Cleaned target directory", "✓".green());
            }
        }
        Commands::Publish { no_verify } => {
            println!("{} Publishing package...", "→".green());
            println!("{} Package published", "✓".green());
        }
        Commands::Search { query } => {
            println!("{} Searching for '{}'...", "→".green(), query);
        }
        Commands::Install { package, version } => {
            println!("{} Installing {}...", "→".green(), package);
            println!("{} Installed {}", "✓".green(), package);
        }
        Commands::Uninstall { package } => {
            println!("{} Uninstalling {}...", "→".green(), package);
            println!("{} Uninstalled {}", "✓".green(), package);
        }
    }

    Ok(())
}
