use std::env;
use std::path::PathBuf;

use normpath::PathExt;

// Lifted from `rust-lang/git2-rs` at 81522979178da3751ba9ebe460f9e45cda706a6e.
/// Tries to use system openvr and emits necessary build script instructions.
fn try_system_openvr() -> Result<pkg_config::Library, pkg_config::Error> {
    let mut cfg = pkg_config::Config::new();
    match cfg.range_version("1.23.7".."1.23.8").probe("openvr") {
        Ok(lib) => {
            for include in &lib.include_paths {
                println!("cargo:root={}", include.display());
            }
            Ok(lib)
        }
        Err(e) => {
            println!("cargo:warning=failed to probe system openvr: {e}");
            Err(e)
        }
    }
}

fn vendored_files() -> Option<Vec<PathBuf>> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            Some(vec![
                relative("openvr/bin/win64/openvr_api.dll"),
                relative("openvr/lib/win64/openvr_api.lib"),
            ])
        } else if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
            Some(vec![relative("openvr/bin/linux64/libopenvr_api.so")])
        } else if #[cfg(all(target_os = "linux", target_arch = "x86"))] {
            Some(vec![relative("openvr/bin/linux32/libopenvr_api.so")])
        } else {
            None
        }
    }
}

const ENV_NO_VENDOR: &str = "OPENVR_NO_VENDOR";

fn main() {
    println!("cargo:rerun-if-env-changed={}", ENV_NO_VENDOR);
    let forced_no_vendor = env::var_os(ENV_NO_VENDOR).map_or(false, |s| s != "0");

    // include path openvr/headers
    let include_paths;

    match (forced_no_vendor, vendored_files()) {
        (false, Some(input_files)) => {
            // Using vendored library.
            include_paths = vec![relative("openvr/headers")];

            let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
            for f in input_files {
                let file_name = f.file_name().unwrap();
                std::fs::copy(&f, out_dir.join(file_name)).unwrap_or_else(|err| {
                    panic!("Failed to copy {:?} to {:?}: {err}", file_name, &out_dir)
                });
            }

            println!("cargo:rustc-link-lib=dylib=openvr_api");
            println!("cargo:rustc-link-search=native={:?}", out_dir);
        }
        _ => {
            // Forced no vendor or no vendored library available.
            match try_system_openvr() {
                Ok(lib) => {
                    include_paths = lib.include_paths;
                }
                result @ Err(_) => {
                    println!(
                        concat!(
                            "cargo:error=No compatible system openvr could be found. To try a ",
                            "vendored version (if available), unset `{0}` or use `{0}=0`.",
                        ),
                        ENV_NO_VENDOR,
                    );
                    result.unwrap();
                    unreachable!();
                }
            }
        }
    }

    // This assumes all your C++ bindings are in main.rs
    let mut b = autocxx_build::Builder::new(relative("src/lib.rs"), include_paths)
        .build()
        .expect("Could not autogenerate bindings");
    // arbitrary library name, pick anything
    b.flag_if_supported("-std=c++14").compile("foobar");
    println!("cargo:rerun-if-changed=src/lib.rs");
}

fn relative(s: &str) -> PathBuf {
    let result = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    result.join(s).normalize().unwrap().into_path_buf()
}
