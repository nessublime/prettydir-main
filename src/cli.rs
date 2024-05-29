use std::path::PathBuf;

use clap::{arg, value_parser, Parser};

#[derive(Parser, Debug)]
pub struct CliArgs {
    /// Maximun file search depth
    #[arg(short, long, default_value_t = 10, value_parser = value_parser!(u8).range(1..24))]
    pub depth: u8,

    /// Path to search from
    pub path: PathBuf,

    /// Paths to exclude
    #[arg(short, long)]
    pub blacklist_patterns: Vec<String>,

    /// Show hidden dot files and folders.
    /// In windows platforms, it will display hidden files
    #[arg(short = 'i', long)]
    pub show_hidden: bool,

    /// Exlucde files listed in root path .gitignore, if found
    #[arg(short = 'g', long)]
    pub use_git_ignore: bool,

    /// Display emoji icons before the file name
    #[arg(short = 'e', long)]
    pub display_emoji: bool,
}

impl CliArgs {
    pub fn parse_opts() -> CliArgs {
        return CliArgs::parse();
    }
}
