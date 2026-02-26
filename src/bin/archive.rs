use clap::Parser;
use archiveundo::{get_snapshot_dir, archive_current_dir};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "archive")]
#[command(about = "Stores a snapshot of your codebase", long_about = None)]
struct Args {
    /// Name of the archive
    name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let target = get_snapshot_dir(&args.name)?;
    
    println!("Archiving current directory to {}...", args.name);
    archive_current_dir(&target)?;
    println!("Done!");
    
    Ok(())
}
