// build.rs

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    #[cfg(feature = "server")]
    {
        let yew_app_dir = format!("{}/yew-app", env::var("CARGO_MANIFEST_DIR").unwrap());

        Command::new("trunk")
            .args(&["build --release"])
            .current_dir(&Path::new(&yew_app_dir))
            .status()
            .unwrap();

        println!("building {}", yew_app_dir);
    }

    println!("cargo:rerun-if-changed=build.rs");
}
