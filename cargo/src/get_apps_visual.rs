use std::process::Command;

/// Calls DNF and returns a formatted string: "app1\napp2\napp3\n"
pub fn get_packages_string() -> Result<String, std::io::Error> {
    let output = Command::new("dnf")
        .args(&[
            "repoquery", 
            "--available", 
            "--queryformat", 
            "%{name}\n"
        ])
        .output()?;

    let raw_stdout = String::from_utf8(output.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;


    if !output.status.success() {
        return Err(std::io::Error::other(
            "dnf repoquery failed",
        ));
    }

    Ok(format_packages(&raw_stdout))
}

fn format_packages(raw_stdout: &str) -> String {
    raw_stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| format!("{line}\n"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::format_packages;

    #[test]
    fn output_packages() {
        assert_eq!(format_packages("app1\napp2\napp3\n"), "app1\napp2\napp3\n");
    }
}
