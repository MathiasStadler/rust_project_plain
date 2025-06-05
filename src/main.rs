use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};
use chrono::Local;

/// Represents project information and system details
#[derive(serde::Serialize, serde::Deserialize)]
struct ProjectInfo {
    name: String,
    start_date: String,
    system_info: HashMap<String, String>,
}

impl ProjectInfo {
    /// Creates a new ProjectInfo instance with system information
    fn new(name: &str) -> Result<Self> {
        let mut system_info = HashMap::new();
        system_info.insert("os".to_string(), std::env::consts::OS.to_string());
        system_info.insert("cpu_cores".to_string(), sys_info::cpu_num()?.to_string());
        system_info.insert("memory_mb".to_string(), (sys_info::mem_info()?.total / 1024).to_string());
        system_info.insert("rust_version".to_string(), rustc_version::version()?.to_string());

        Ok(ProjectInfo {
            name: name.to_string(),
            start_date: Local::now().format("%Y-%m-%d").to_string(),
            system_info,
        })
    }

    /// Generates markdown content for project_path.md
    fn generate_markdown(&self) -> String {
        format!(
            "# Project: {}\n\n\
             ## Project Information\n\n\
             - Start Date: {}\n\
             - OS: {}\n\
             - Hardware:\n\
               - CPU Cores: {}\n\
               - Memory: {} MB\n\
             - Rust Version: {}\n",
            self.name,
            self.start_date,
            self.system_info.get("os").unwrap_or(&"unknown".to_string()),
            self.system_info.get("cpu_cores").unwrap_or(&"unknown".to_string()),
            self.system_info.get("memory_mb").unwrap_or(&"unknown".to_string()),
            self.system_info.get("rust_version").unwrap_or(&"unknown".to_string()),
        )
    }

    /// Generates changelog.md content
    fn generate_changelog(&self) -> String {
        format!(
            "# Changelog\n\n\
             ## [0.1.0] - {}\n\n\
             ### Added\n\
             - Initial project setup\n\
             - Basic project structure\n\
             - System information collection\n",
            self.start_date
        )
    }
}

/// Creates a new project folder with required files
fn create_project(name: &str) -> Result<()> {
    let project_path = PathBuf::from("/tmp").join(name);

    // Check if folder already exists
    if project_path.exists() {
        anyhow::bail!("Project folder '{}' already exists", name);
    }

    // Create project directory
    fs::create_dir_all(&project_path)
        .context("Failed to create project directory")?;

    let project_info = ProjectInfo::new(name)?;

    // Create project_path.md
    fs::write(
        project_path.join("project_path.md"),
        project_info.generate_markdown(),
    ).context("Failed to write project_path.md")?;

    // Create changelog.md
    fs::write(
        project_path.join("changelog.md"),
        project_info.generate_changelog(),
    ).context("Failed to write changelog.md")?;

    println!("✅ Project '{}' created successfully in /tmp!", name);
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("❌ Usage: {} <project_name>", args[0]);
        std::process::exit(1);
    }

    if let Err(e) = create_project(&args[1]) {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;

    #[test]
    fn test_project_creation() -> Result<()> {
        let test_project = "test_project";
        let project_path = PathBuf::from("/tmp").join(test_project);

        // Clean up if exists
        let _ = fs::remove_dir_all(&project_path);

        // Create project
        create_project(test_project)?;

        // Verify files exist
        assert!(project_path.exists());
        assert!(project_path.join("project_path.md").exists());
        assert!(project_path.join("changelog.md").exists());

        // Clean up
        fs::remove_dir_all(project_path)?;
        Ok(())
    }

    #[test]
    fn test_existing_folder_error() {
        let test_project = "test_existing";
        let project_path = PathBuf::from("/tmp").join(test_project);

        // Create folder first
        fs::create_dir_all(&project_path).unwrap();

        // Try to create project
        let result = create_project(test_project);
        assert!(result.is_err());

        // Clean up
        fs::remove_dir_all(project_path).unwrap();
    }

    #[test]
    fn test_cli() {
        let mut cmd = Command::cargo_bin("rust_project_plain").unwrap();
        cmd.arg("test_cli_project")
            .assert()
            .success();

        // Clean up
        fs::remove_dir_all("/tmp/test_cli_project").unwrap();
    }
}