use std::path::PathBuf;

use clap::Parser;
use cli::dump;

#[derive(Parser)]
pub struct Args {
    /// File to dump
    pub file: String,
    /// Amount of lines displayed per-page
    #[arg(short, long)]
    pub line_length: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let path = args.file;
    let path = PathBuf::from(path);
    dump(path, args.line_length);
}
