// build.rs

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let app_dir = format!("{}/app", env::var("CARGO_MANIFEST_DIR").unwrap());

    let status = Command::new("trunk")
        .args(&["build", "--release"])
        .current_dir(&Path::new(&app_dir))
        .status();

    match status {
        Ok(status) if status.success() => println!("Successfully built {}", app_dir),
        Ok(status) => eprintln!("Failed to build {}: exit code {}", app_dir, status),
        Err(err) => eprintln!("Failed to run trunk command: {}", err),
    }
    println!("cargo:rerun-if-changed=build.rs");
}
