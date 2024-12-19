//! This goes in your build script, for use with react-scripts,
//! and cra template projects.
//!
//! It can be helpful for both development and deployment.
//!
//! So, as an example:
//! ```
//! use build_my_react_js::*;
//!
//! fn main() {
//!     build_my_react_js("my-frontend", env!("CARGO_MANIFEST_DIR"));
//! }
//! ```
//!
//! Provided the system is configured with NPM, and the repositories
//! are reachable, then it will attempt to compile your React project
//! when changes are detected (and only when changes are detected)
//!
//! Provided your crate is structured with an additional, uniquely-named
//! subdirectory containing your `package.json` this can be instructed
//! to enter and build it. Feedback is provided through the Cargo IPC
//! mechanism as build warnings. Panics are fatal to builds, when the
//! commands report failure.
//!
//! I'm not sure how "clean" works in the npm ecosystem, but this crate
//! assumes you start potentially without node_modules, and attempts
//! npm install when the project is not built yet.
//!
//! This could become flaky, but:
//! - attempts to preserve quality feedback,
//! - rely on quality sources of information,
//! - deliver quality feedback,
//! - benefit from pipelined builds to speed development,
//! - benefit from the full power of the cargo ecosystem
//!
//! Enjoy!
//! 

use core::str;
use inline_colorization::*;
use std::{path::PathBuf, process::Command};

pub fn build_my_react_js(path: &str, outer_env: &str) {
    match build_my_react_js_fallible(path, outer_env, false) {
        Ok(_) => (),
        Err(err_msg) => panic!("{err_msg}"),
    }
}

pub fn build_my_react_js_silent(path: &str, outer_env: &str) {
    match build_my_react_js_fallible(path, outer_env, true) {
        Ok(_) => (),
        Err(err_msg) => panic!("{err_msg}"),
    }
}

pub fn build_my_react_js_fallible(path: &str, outer_env: &str, silent: bool) -> Result<(), String> {
    let mut d = PathBuf::from(outer_env);
    d.push(format!("{path}/build/index.html"));
    match std::fs::exists(PathBuf::from(d)) {
        Ok(defined) => {
            if defined {
                let mut d = PathBuf::from(outer_env);
                d.push(format!("{path}/src/"));
                println!("cargo::rerun-if-changed={}", d.to_string_lossy());

                let mut d = PathBuf::from(outer_env);
                d.push(format!("{path}/package.json"));
                println!("cargo::rerun-if-changed={}", d.to_string_lossy());
            } else {
                let mut d = PathBuf::from(outer_env);
                d.push(format!("{path}/"));
                if let Ok(output) = Command::new("npm").arg("ping").output() {
                    if !output.status.success() {
                        print_warning(
                            format!("Unable to locate npm, cannot complete build."),
                            silent,
                        );

                        print_warning(
                            format!("Failed with: {}", str::from_utf8(&output.stdout).unwrap()),
                            silent,
                        );
                        return Err("NPM unavailable".to_string());
                    } else {
                        print_warning(format!("Located NPM for frontend build."), silent);
                    }
                } else {
                    return Err(format!("{style_bold}{color_bright_red}ReactJS Frontend build error:{color_reset}{style_reset} Node Package Manager not found, or npm registry unreachable! Ensure the system is configured with npm."));
                }

                let mut d = PathBuf::from(outer_env);
                d.push(format!("{path}/"));
                if let Ok(output) = Command::new("npm")
                    .current_dir(d)
                    .arg(format!("install"))
                    .output()
                {
                    if !output.status.success() {
                        print_warning(format!("NPM build failed."), silent);
                        print_warning(
                            format!(
                                "NPM build reported:{}",
                                str::from_utf8(&output.stdout).unwrap()
                            ),
                            silent,
                        );
                        return Err("NPM unavailable".to_string());
                    } else {
                        print_warning(format!("Installed **node_modules**"), silent);
                    }
                } else {
                    return Err("Node Package Manager error! Check system logs.".to_string());
                }
            }
        }
        Err(e) => return Err(e.to_string()),
    }

    let mut d = PathBuf::from(outer_env);
    d.push(format!("{path}/"));
    if let Ok(output) = Command::new("npm").arg("ping").output() {
        if !output.status.success() {
            print_warning(
                format!("Unable to locate npm, cannot complete build."),
                silent,
            );
            print_warning(
                format!("Failed with: {}", str::from_utf8(&output.stdout).unwrap()),
                silent,
            );
            return Err("NPM unavailable".to_string());
        } else {
            print_warning(format!("Located NPM for frontend build."), silent);
        }
    } else {
        return Err(
            "Node Package Manager not found! Ensure the system is configured with npm.".to_string(),
        );
    }

    let mut d = PathBuf::from(outer_env);
    d.push(format!("{path}/"));
    if let Ok(output) = Command::new("npm")
        .current_dir(d)
        .arg("run")
        .arg("build")
        .output()
    {
        if !output.status.success() {
            print_warning(format!("NPM build failed."), silent);
            print_warning(
                format!(
                    "NPM build reported:{}",
                    str::from_utf8(&output.stdout).unwrap()
                ),
                silent,
            );
            return Err("NPM unavailable".to_string());
        } else {
            print_warning(format!("Frontend build completed successfully!"), silent);
        }
    } else {
        return Err("Node Package Manager error! Check system logs.".to_string());
    }

    Ok(())
}

fn print_warning(s: String, silent: bool) {
    if !silent {
        println!("cargo::warning={}", s);
    }
}
