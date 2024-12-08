use std::env;
use std::fs;
use std::process::{exit, Command};

fn main() {
    let mut date_value = 1;

    if let Ok(entries) = fs::read_dir("example") {
        for entry in entries.filter_map(Result::ok) {
            if let Some(name) = entry.file_name().to_str() {
                if let Some(value) = name.strip_prefix("day") {
                    if let Some(value) = value.strip_suffix(".txt") {
                        if let Ok(num) = value.parse::<usize>() {
                            if date_value < num + 1 {
                                date_value = num + 1;
                            }
                        }
                    }
                }
            }
        }
    }

    let gen = env::var("GEN").is_ok();
    if gen {
        // Call the gen.sh script with the value of DATE and capture its output
        let output = Command::new("./gen.sh")
            .arg(format!("{}", date_value)) // Pass the date value to the script
            .output()
            .expect("Failed to execute gen.sh");

        // Check if the script executed successfully
        if !output.status.success() {
            eprintln!(
                "Error: gen.sh script failed with exit code: {}",
                output.status.code().unwrap_or(1)
            );
            exit(1); // Exit with an error status
        }

        // Print the stdout from gen.sh
        println!("{}", String::from_utf8_lossy(&output.stdout));

        // Optionally print stderr from gen.sh if needed
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));

        // Optionally print the date to the build logs for confirmation
        println!("cargo:warning=Setting DATE to: {}", date_value);
    }

    // Set the environment variable DATE to the selected value
    println!("cargo:env=DATE={}", date_value); // Export DATE variable
    println!("cargo:rerun-if-changed=build.rs"); // Ensure build.rs reruns if it changes
}
