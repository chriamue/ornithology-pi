// build.rs

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    #[cfg(feature = "server")]
    {
        let app_dir = format!("{}/app", env::var("CARGO_MANIFEST_DIR").unwrap());

        Command::new("trunk")
            .args(&["build --release"])
            .current_dir(&Path::new(&app_dir))
            .status()
            .unwrap();

        println!("building {}", app_dir);
    }

    println!("cargo:rerun-if-changed=build.rs");
}
