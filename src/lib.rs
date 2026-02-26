use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use fs_extra::dir::CopyOptions;

pub fn get_snapshot_base_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let base_dir = home.join(".archiveundo").join("snapshots");
    if !base_dir.exists() {
        std::fs::create_dir_all(&base_dir)?;
    }
    Ok(base_dir)
}

pub fn get_snapshot_dir(name: &str) -> Result<PathBuf> {
    let base = get_snapshot_base_dir()?;
    Ok(base.join(name))
}

pub fn archive_current_dir(target_dir: &Path) -> Result<()> {
    if target_dir.exists() {
        std::fs::remove_dir_all(target_dir).context("Failed to remove existing snapshot")?;
    }
    std::fs::create_dir_all(target_dir).context("Failed to create target directory")?;

    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    
    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.content_only = true;

    // We use a manual walk to skip .git if we want more control, 
    // but for now let's just copy everything and then maybe remove .git if it exists in the target.
    // Or better, use a library that supports filtering.
    // Since fs_extra is simple, let's just copy and then remove .git from target.
    
    fs_extra::dir::copy(&current_dir, target_dir, &options)
        .context("Failed to copy directory")?;

    let git_dir = target_dir.join(".git");
    if git_dir.exists() {
        std::fs::remove_dir_all(git_dir).ok();
    }

    Ok(())
}

pub fn restore_from_snapshot(snapshot_dir: &Path) -> Result<()> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    
    // Safety check: don't delete everything if the snapshot doesn't exist
    if !snapshot_dir.exists() {
        anyhow::bail!("Snapshot does not exist at {:?}", snapshot_dir);
    }

    // Remove everything in current dir except .git if we want to preserve it?
    // The user said "restore your codebase to that snapshot".
    // Usually this means wiping the current state.
    
    for entry in std::fs::read_dir(&current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            std::fs::remove_dir_all(&path)?;
        } else {
            std::fs::remove_file(&path)?;
        }
    }

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.content_only = true;
    
    fs_extra::dir::copy(snapshot_dir, &current_dir, &options)
        .context("Failed to restore from snapshot")?;

    Ok(())
}
