use std::{env, process};
use chrono::Local;
use obsidian_note_rust::build_content;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: note <text...>");
        process::exit(1);
    }

    let timestamp = Local::now().format("%Y-%m-%d %H:%M").to_string();
    let content = build_content(&timestamp, &args);

    let status = process::Command::new("obsidian")
        .args(["append", "file=inbox", &format!("content={}", content)])
        .status()
        .expect("failed to run obsidian CLI — is it installed?");

    process::exit(status.code().unwrap_or(1));
}
