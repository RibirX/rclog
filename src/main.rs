mod util;
use clap::{Parser, Subcommand};
use semver::Version;
use util::*;

/// A tool designed to streamline your development process
/// by merging pre-release version changelogs and extracting
/// the content of a specific version.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// What you want to do with the changelog.
  #[command(subcommand)]
  command: Commands,
  /// Specify the version of the changelog you wish to manipulate.
  #[arg(short, long, value_name = "VERSION")]
  target: String,
  /// The content of the changelog. Use `path` instead if you want to read from a file.
  #[arg(short, long)]
  content: Option<String>,
  /// The path to the changelog file.
  #[arg(short, long, value_name = "FILE")]
  path: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
  /// Extract the content of a specific version.
  Extract,
  /// Merge all changelog of pre-release versions of the target version into one.
  Merge,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();
  let version = Version::parse(&args.target)?;
  let changelog = match (args.content, args.path) {
    (Some(content), None) => content,
    (None, Some(path)) => std::fs::read_to_string(path)?,
    _ => {
      return Err("You must specify either `content` or `path`.".into());
    }
  };
  match args.command {
    Commands::Extract => {
      let content = extract_content(&version, &changelog);
      println!("{}", content);
    }
    Commands::Merge => {
      let content = merge_pre_release_changelogs(&version, &changelog)?;
      println!("{}", content);
    }
  }
  Ok(())
}
