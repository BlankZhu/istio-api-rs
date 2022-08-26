use std::env;
use std::fs;
use std::os::unix::fs::symlink;
use std::path;

const SRC: &str = "src";

#[cfg(feature = "v1_10")]
const FEATURE: &str = "v1_10";

#[cfg(feature = "v1_11")]
const FEATURE: &str = "v1_11";

#[cfg(feature = "v1_12")]
const FEATURE: &str = "v1_12";

#[cfg(feature = "v1_13")]
const FEATURE: &str = "v1_13";

#[cfg(feature = "v1_14")]
const FEATURE: &str = "v1_14";

fn main() -> std::io::Result<()> {
    // get current working directory
    let current_dir = match env::current_dir() {
        Ok(cd) => cd,
        Err(e) => panic!("failed to get current working directory, detail: {}", e),
    };
    println!("current working directory: {}", current_dir.display());

    // change directory to `src/`
    let test_src_dir = current_dir.join(SRC);
    if let Err(e) = env::set_current_dir(test_src_dir.as_path()) {
        panic!(
            "failed to change current working directory: {:?}, detail: {}",
            test_src_dir.as_path(),
            e
        );
    }

    // make up path to versioned directory
    let versioned_dir = path::Path::new(FEATURE);
    let paths = match fs::read_dir(versioned_dir) {
        Ok(ps) => ps,
        Err(e) => panic!("failed to read directory {}, detail: {}", FEATURE, e),
    };

    // make up symbolic links
    for path in paths {
        if let Ok(entry) = path {
            if entry.path().is_dir() {
                let src_p_str = entry.path().display().to_string();
                let dest_p_str = entry.file_name();

                if path::Path::new(&dest_p_str).exists() {
                    if let Err(e) = fs::remove_file(&dest_p_str) {
                        panic!("failed to remove old existing file {:?}, detail: {}", dest_p_str, e);
                    }
                }

                println!("symbolic linking {} to {:?}", src_p_str, entry.file_name());
                let sl_res = symlink(src_p_str.as_str(), entry.file_name());
                if let Err(err) = sl_res {
                    panic!(
                        "failed to create symbolic link for {} on {:?}, detail: {}",
                        src_p_str, dest_p_str, err
                    );
                }
            }
            if entry.path().is_file() {
                let src_p_str = entry.path().display().to_string();
                if src_p_str.contains("mod.rs") {
                    let dest_p_str = "lib.rs";
                    // check and remove `lib.rs` if it exists
                    if path::Path::new("lib.rs").exists() {
                        if let Err(e) = fs::remove_file("lib.rs") {
                            panic!("failed to remove old existing lib.rs, detail: {}", e);
                        }
                    }

                    println!("symbolic linking {} to {:?}", src_p_str, dest_p_str);
                    let sl_res = symlink(src_p_str.as_str(), dest_p_str);
                    if let Err(err) = sl_res {
                        panic!(
                            "failed to create symbolic link for {} on {}, detail: {}",
                            src_p_str, dest_p_str, err
                        )
                    }
                }
            }
        }
    }

    env::set_current_dir(current_dir)
}
