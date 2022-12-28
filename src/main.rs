use crate::{
    args::Args,
    cargo::parse_manifest,
    git::{
        check_staged_files_exist, commit_to_repo, generate_commit_msg, get_repository,
        DEFAULT_TYPES,
    },
    questions::{ask, SurveyResults},
};
use clap::Parser;
use std::{collections::HashMap, path::Path};

mod args;
mod cargo;
mod git;
mod questions;

fn run_dialog() -> Option<SurveyResults> {
    let manifest = parse_manifest().unwrap();
    if let Some(package) = manifest.package {
        if let Some(metadata) = package.metadata {
            // Use default scopes and/or custom ones.
            let mut types: HashMap<&str, &str> = HashMap::with_capacity(10);
            if metadata.commits.defaults {
                types.extend(&*DEFAULT_TYPES);
            }

            // Insert custom types.
            if let Some(custom_types) = &metadata.commits.r#type {
                for r#type in custom_types.iter() {
                    types.insert(&r#type.name, &r#type.desc);
                }
            }

            return Some(ask(types));
        } else {
            eprintln!("Please specify allowed scopes inside of your Cargo.toml file under the `package.metadata.cz` key!");
        }
    }

    None
}

fn create_commit(commit_msg: &str, repo: &Path) {
    let hash = commit_to_repo(commit_msg, repo).expect("Failed to create commit");
    println!("Wrote commit: {}", hash);
}

fn run(args: Args) {
    // No point to continue if repo doesn't exist or there are no staged files
    if check_staged_files_exist(args.repo_path.as_path()) {
        let survey = run_dialog();
        let commit_msg = survey.map(generate_commit_msg).and_then(|msg| {
            if args.edit {
                edit::edit(msg).ok()
            } else {
                Some(msg)
            }
        });

        match commit_msg {
            Some(msg) => create_commit(&msg, args.repo_path.as_path()),
            None => eprintln!("Empty commit message specified!"),
        }
    } else {
        eprintln!("Nothing to commit!");
    }
}

fn main() {
    let args: Args = Args::parse();
    // Early return if the path doesn't exist.
    if !args.repo_path.exists() || get_repository(args.repo_path.as_path()).is_err() {
        eprintln!("Invalid path to repository: {}", args.repo_path.display());
    } else {
        // When terminating the CLI during the dialoguer phase, the cursor will be
        // hidden. The callback here makes sure that the cursor is visible in these
        // cases.
        let _ = ctrlc::set_handler(move || {
            let term = dialoguer::console::Term::stderr();
            let _ = term.show_cursor();
            std::process::exit(1);
        });

        run(args);
    }
}
