use std::process::Command;

pub fn get_info(package: &str) -> Result<Vec<String>, std::io::Error> {
	let output = Command::new("dnf")
		.args(["info", package])
		.output()?;

	Ok(extract_description(&String::from_utf8_lossy(&output.stdout)))
}

fn extract_description(output: &str) -> Vec<String> {
	let mut description_started = false;
	let mut description = Vec::new();

	for line in output.lines() {
		if !description_started {
			if line.starts_with("Description") {
				description_started = true;
				let first_line = line
					.strip_prefix("Description")
					.unwrap_or(line)
					.trim_start();
				let first_line = first_line
					.strip_prefix(':')
					.unwrap_or(first_line)
					.trim();
				if !first_line.is_empty() {
					description.push(first_line.to_owned());
				}
			}
			continue;
		}

		if line.starts_with("Vendor") {
			break;
		}

		description.push(line.to_owned());
	}

	if description.is_empty() {
		vec!["This app dosen't have a description".to_owned()]
	} else {
		description
	}
}

#[cfg(test)]
mod tests {
	use super::extract_description;

	#[test]
	fn returns_lines_after_description_header() {
		let output = "Name : sample\nDescription : sample package\n  Details here\nURL : example\nVendor : Example\nPackager : Example";

		assert_eq!(
			extract_description(output),
			["sample package", "  Details here", "URL : example"]
		);
	}

	#[test]
	fn returns_fallback_when_description_is_empty() {
		assert_eq!(
			extract_description(""),
			["This app dosen't have a description"]
		);
	}
}