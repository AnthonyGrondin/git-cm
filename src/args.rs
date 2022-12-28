use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Args {
    /// Opens the user's editor after the questioning process.
    #[arg(long, short = 'e')]
    pub edit: bool,
    /// The path to a git repository.
    #[arg(name = "REPO", default_value = ".", value_parser)]
    pub repo_path: PathBuf,
}
