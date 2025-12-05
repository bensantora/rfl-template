use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::templates;

pub struct ModuleConfig {
    pub name: String,
    pub author: String,
    pub description: String,
    pub license: String,
}

pub fn generate_module(config: ModuleConfig) -> Result<()> {
    let module_dir = Path::new(&config.name);

    // Check if directory already exists
    if module_dir.exists() {
        anyhow::bail!("Directory '{}' already exists", config.name);
    }

    // Validate module name (must be valid Rust identifier with underscores)
    if !is_valid_module_name(&config.name) {
        anyhow::bail!(
            "Invalid module name '{}'. Must contain only lowercase letters, numbers, and underscores, and start with a letter.",
            config.name
        );
    }

    // Create directory structure
    fs::create_dir(&module_dir)
        .context("Failed to create module directory")?;
    
    let src_dir = module_dir.join("src");
    fs::create_dir(&src_dir)
        .context("Failed to create src directory")?;

    // Generate files
    generate_cargo_toml(&module_dir, &config)?;
    generate_lib_rs(&src_dir, &config)?;
    generate_kbuild(&module_dir, &config)?;
    generate_readme(&module_dir, &config)?;

    println!("✓ Created kernel module '{}'", config.name);
    println!("\nNext steps:");
    println!("  cd {}", config.name);
    println!("  # Link into your kernel source tree");
    println!("  # See README.md for build instructions");

    Ok(())
}

fn generate_cargo_toml(dir: &Path, config: &ModuleConfig) -> Result<()> {
    let content = templates::cargo_toml::generate(&config.name);
    let path = dir.join("Cargo.toml");
    fs::write(&path, content)
        .with_context(|| format!("Failed to write {:?}", path))?;
    Ok(())
}

fn generate_lib_rs(dir: &Path, config: &ModuleConfig) -> Result<()> {
    let content = templates::lib_rs::generate(
        &config.name,
        &config.author,
        &config.description,
        &config.license,
    );
    let path = dir.join("lib.rs");
    fs::write(&path, content)
        .with_context(|| format!("Failed to write {:?}", path))?;
    Ok(())
}

fn generate_kbuild(dir: &Path, config: &ModuleConfig) -> Result<()> {
    let content = templates::kbuild::generate(&config.name);
    let path = dir.join("Kbuild");
    fs::write(&path, content)
        .with_context(|| format!("Failed to write {:?}", path))?;
    Ok(())
}

fn generate_readme(dir: &Path, config: &ModuleConfig) -> Result<()> {
    let content = templates::readme::generate(&config.name, &config.description);
    let path = dir.join("README.md");
    fs::write(&path, content)
        .with_context(|| format!("Failed to write {:?}", path))?;
    Ok(())
}

fn is_valid_module_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let mut chars = name.chars();
    
    // First character must be a letter
    if !chars.next().map_or(false, |c| c.is_ascii_lowercase()) {
        return false;
    }

    // Rest can be letters, numbers, or underscores
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}
