use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

fn get_available_packages() -> Result<Vec<String>, std::io::Error> {
    // Invoke repoquery directly without a shell wrapper
    let mut child = Command::new("dnf")
        .args(&["repoquery", "--available", "--queryformat", "%{name}.%{arch}"])
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Could not capture stdout")
    })?;

    let reader = BufReader::new(stdout);
    
    // Efficiently stream each line into a Vector
    let packages: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty()) // Filter out any empty trailing lines
        .collect();

    Ok(packages)
}

fn main() {
    if let Ok(pkgs) = get_available_packages() {
        // If you absolutely need it as a JSON string for a file/config:
        let json_array = serde_json::to_string(&pkgs).unwrap();
        println!("Loaded {} packages.", pkgs.len());
    }
}
