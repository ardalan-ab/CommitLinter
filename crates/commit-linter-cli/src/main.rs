mod config;
mod parser;
mod reporter;
mod rules;

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use crate::config::{init_config, load_config};
use crate::parser::parse_header;
use crate::reporter::{print_errors, print_success};
use crate::rules::validate;

#[derive(Debug, Parser)]
#[command(name = "commit-linter-rs")]
#[command(about = "A universal Conventional Commit message linter")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Lint a commit message.
    Lint {
        /// Commit message text to lint.
        #[arg(long, conflicts_with = "edit")]
        message: Option<String>,

        /// Path to a Git commit message file.
        #[arg(long)]
        edit: Option<PathBuf>,
    },

    /// Generate commit-linter.toml in the current directory.
    Init,

    /// Install a Git commit-msg hook in the current repository.
    Install,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lint { message, edit } => lint(message, edit),
        Commands::Init => {
            let created = init_config()?;
            if created {
                println!("Created commit-linter.toml");
            } else {
                println!("commit-linter.toml already exists");
            }
            Ok(())
        }
        Commands::Install => install_hook(),
    }
}

fn lint(message: Option<String>, edit: Option<PathBuf>) -> Result<()> {
    let message = match (message, edit) {
        (Some(message), None) => message,
        (None, Some(path)) => fs::read_to_string(&path)
            .with_context(|| format!("failed to read commit message file {}", path.display()))?,
        (None, None) => anyhow::bail!("provide either --message or --edit"),
        (Some(_), Some(_)) => anyhow::bail!("provide only one of --message or --edit"),
    };

    let header = first_header_line(&message);
    let config = load_config()?;
    let parsed = parse_header(header);
    let errors = validate(header, parsed.as_ref(), &config);

    if errors.is_empty() {
        print_success();
        Ok(())
    } else {
        print_errors(&errors);
        std::process::exit(1);
    }
}

fn first_header_line(message: &str) -> &str {
    message.lines().next().unwrap_or("").trim_end()
}

fn install_hook() -> Result<()> {
    let git_dir = PathBuf::from(".git");
    if !git_dir.is_dir() {
        anyhow::bail!("could not find .git directory in the current working directory");
    }

    let hooks_dir = git_dir.join("hooks");
    fs::create_dir_all(&hooks_dir).context("failed to create .git/hooks directory")?;

    let hook_path = hooks_dir.join("commit-msg");
    fs::write(
        &hook_path,
        "#!/bin/sh\ncommit-linter-rs lint --edit \"$1\"\n",
    )
    .with_context(|| format!("failed to write {}", hook_path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = fs::metadata(&hook_path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&hook_path, permissions)?;
    }

    println!("Installed Git commit-msg hook at {}", hook_path.display());
    let created_config = init_config()?;
    if created_config {
        println!("Created commit-linter.toml");
    } else {
        println!("commit-linter.toml already exists");
    }
    Ok(())
}
