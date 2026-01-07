//! Build script to copy assets from workspace root to crate directory
//! 
//! This ensures Dioxus can find assets during compilation.
//! Assets are copied from the workspace root `assets/` directory to
//! `crates/web/assets/` before compilation.

use std::fs;
use std::path::Path;

fn main() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let crate_path = Path::new(crate_dir);
    
    // Calculate workspace root: crates/web -> crates -> workspace root
    let workspace_root = crate_path
        .parent()  // crates/web -> crates
        .and_then(|p| p.parent())  // crates -> workspace root
        .expect("Failed to find workspace root");
    
    let assets_src = workspace_root.join("assets");
    let assets_dst = crate_path.join("assets");

    // Skip if source doesn't exist
    if !assets_src.exists() {
        eprintln!("Warning: Assets directory not found at {:?}", assets_src);
        return;
    }

    // Remove existing symlink if present
    if assets_dst.exists() && assets_dst.is_symlink() {
        let _ = fs::remove_file(&assets_dst);
    }

    // Copy assets directory
    if let Err(e) = copy_dir_all(&assets_src, &assets_dst) {
        eprintln!("Warning: Failed to copy assets: {}", e);
    } else {
        println!("cargo:rerun-if-changed={}", assets_src.display());
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    // Remove destination if it exists (to handle updates)
    if dst.exists() && !dst.is_symlink() {
        fs::remove_dir_all(dst)?;
    }
    
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

