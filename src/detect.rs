use serde_json::Value;
use std::{error::Error, fs, path::Path};

fn get_package_json() -> Result<Value, Box<dyn Error>> {
    let contents = fs::read_to_string("package.json")?;
    let json: Value = serde_json::from_str(&contents)?;
    Ok(json)
}

fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn from_package_json() -> Result<String, Box<dyn Error>> {
    let json = get_package_json()?;
    let package_manager = json["packageManager"]
        .as_str()
        .ok_or("Package manager not found in package.json")?;

    let package_manager: Vec<&str> = package_manager.split('@').collect();
    let agent = if package_manager.first().unwrap() == &"yarn"
        && package_manager[1].parse::<i32>().unwrap() > 1
    {
        "berry"
    } else {
        package_manager.first().unwrap()
    };
    Ok(agent.to_owned())
}

fn from_lock_file() -> Result<String, Box<dyn Error>> {
    let package_manager = if file_exists("package-lock.json") {
        "npm"
    } else if file_exists("yarn.lock") {
        "yarn"
    } else if file_exists("pnpm-lock.yaml") {
        "pnpm"
    } else if file_exists("bun.lockb") {
        "bun"
    } else if file_exists("poetry.lock") {
        "poetry"
    } else if file_exists("requirements.txt") {
        "pip"
    } else if file_exists("Cargo.lock") {
        "cargo"
    } else {
        return Err(From::from("No package manager found"));
    };
    Ok(package_manager.to_owned())
}

pub fn detect_package_manager() -> String {
    return if let Ok(package_manager) = from_package_json() {
        package_manager
    } else {
        from_lock_file().unwrap()
    };
}
