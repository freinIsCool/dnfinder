use std::process::Command;

/// Calls DNF and returns package names formatted as an array-like string.
pub fn get_packages_string_v() -> Result<String, std::io::Error> {
    let output = Command::new("dnf")
        .args(&[
            "repoquery", 
            "--available", 
            "--queryformat", 
            "%{name} "
        ])
        .output()?;

    let raw_stdout = String::from_utf8(output.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    Ok(format_packages_v(&raw_stdout))
}

fn format_packages_v(raw_stdout: &str) -> String {
    let packages: Vec<String> = raw_stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| format!("{line:?}"))
        .collect();

    format!("[{}]", packages.join(", "))
}

#[cfg(test)]
mod tests {
    use super::format_packages_v;

    #[test]
    fn outputPackages_v() {
        assert_eq!(format_packages_v("app1\napp2\napp3\n"), "[\"app1\", \"app2\", \"app3\"]");
    }
}
