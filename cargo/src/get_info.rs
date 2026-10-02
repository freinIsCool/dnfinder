use std::process::Command;

pub fn get_info(package: &str) -> Result<Vec<String>, std::io::Error> {
	let output = Command::new("dnf")
		.args(["info", package])
		.output()?;

	Ok(extract_description(&String::from_utf8_lossy(&output.stdout)))
}

fn extract_description(output: &str) -> Vec<String> {
	let mut description_started = false;

	let description: Vec<String> = output
		.lines()
		.filter_map(|line| {
			if !description_started && line.starts_with("Description") {
				description_started = true;
				None
			} else if description_started {
				Some(line.to_owned())
			} else {
				None
			}
		})
		.take_while(|line| !line.starts_with("Vendor"))
		.collect();

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

		assert_eq!(extract_description(output), ["  Details here", "URL : example"]);
	}

	#[test]
	fn returns_fallback_when_description_is_empty() {
		assert_eq!(
			extract_description(""),
			["This app dosen't have a description"]
		);
	}
}