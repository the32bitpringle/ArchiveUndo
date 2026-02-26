use clap::Parser;
use archiveundo::{get_snapshot_dir, restore_from_snapshot};
use anyhow::Result;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "undo")]
#[command(about = "Restores your codebase from a snapshot", long_about = None)]
struct Args {
    /// Name of the archive to restore
    name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let snapshot_dir = get_snapshot_dir(&args.name)?;
    
    if !snapshot_dir.exists() {
        println!("Error: Snapshot '{}' not found.", args.name);
        std::process::exit(1);
    }

    print!("This will OVERWRITE your current directory. Are you sure? (y/N): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if input.trim().to_lowercase() != "y" {
        println!("Aborted.");
        return Ok(());
    }

    println!("Restoring from snapshot {}...", args.name);
    restore_from_snapshot(&snapshot_dir)?;
    println!("Done!");
    
    Ok(())
}
