use std::process::Command;

pub fn get_packages() -> Result<Vec<String>, std::io::Error> {
    let output = Command::new("dnf")
        .args([
            "repoquery",
            "--available",
            "--queryformat",
            "%{name}\n",
        ])
        .output()?;

    let raw_stdout = String::from_utf8(output.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    Ok(format_packages(&raw_stdout))
}

fn format_packages(raw_stdout: &str) -> Vec<String> {
    raw_stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}