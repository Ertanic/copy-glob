//! # About it
//! 
//! **copy-glob** is a small utility to copy files to an output directory using glob syntax. Inspired by [copy_to_output](https://crates.io/crates/copy_to_output).
//! 
//! # How to use
//! 
//! Add a crate to the dev dependencies in `Cargo.toml`.
//! 
//! ```toml
//! [dev-dependencies]
//! copy-glob = "0.1"
//! ```
//! 
//! Create `build.rs` and add the following to it:
//! 
//! ```rust
//! use copy_glob::{get_target_folder, copy_glob};
//! 
//! fn main() {
//!     let output = get_target_folder().join("copies"); // target/{debug,release}/copies
//!     copy_glob("**/for_copy/*", output);
//! }
//! ```
//! 
//! This will copy all the files in the directory, preserving the structure, since the copying starts from the root of the project (where the `Cargo.toml` file is located).
//! 
//! To localize the file search somehow, you can use the following code:
//! 
//! ```rust,no_run
//! use copy_glob::{get_target_folder, get_root_path, CopyGlobOptionsBuilder, copy_glob_with};
//! 
//! fn main() {
//!     let output = get_target_folder().join("copies");
//!     let root = get_root_path().join("for_copy"); // same level as Cargo.toml
//!     let options = CopyGlobOptionsBuilder::new().set_root_path(root).build();
//!     copy_glob_with("*.toml", output, &options);
//! }
//! ```
//! 
//! This will copy all files with the `.toml` extension in the `for_copy` folder without preserving the structure.
//! 
//! In addition, you can add exceptions from where files will be copied. By default, the `target` directory hangs in the excludes.
//! 
//! ```rust
//! # use copy_glob::CopyGlobOptionsBuilder;
//! let options = CopyGlobOptionsBuilder::new().add_exclude("**/some_pattern/*").build();
//! ```

use globset::{Glob, GlobMatcher, GlobSet, GlobSetBuilder};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn copy_glob(glob: impl AsRef<str>, output_dir: impl AsRef<Path>) {
    let options = CopyGlobOptionsBuilder::new().build();
    copy_glob_with(glob, output_dir, &options);
}

pub fn copy_glob_with(
    glob: impl AsRef<str>,
    output_dir: impl AsRef<Path>,
    options: &CopyGlobOptions,
) {
    let glob = glob.as_ref();
    let matcher = new_glob(glob);
    let current_dir = options.root.as_ref();

    create_dir_all(&output_dir).unwrap();

    for entry in WalkDir::new(&current_dir) {
        let entry = entry.expect("Unable to read dir");
        if options.exclude.is_match(entry.path()) || !matcher.is_match(entry.path()) {
            continue;
        }

        let path = entry.path().display();
        println!("cargo:rerun-if-changed={path}");

        let metadata = entry.metadata().expect("Unable to get metadata");
        let dirname = get_str(entry.path())
            .strip_prefix(get_str(current_dir))
            .expect("Unable to strip a string")
            .trim_start_matches("\\")
            .trim_start_matches("/");
        let output_path = output_dir.as_ref().join(dirname);

        if metadata.is_dir() {
            create_dir_all(output_path).expect("Unable to create output dir");
        } else if metadata.is_file() {
            let parent = output_path.parent().unwrap();

            if parent.exists() {
                std::fs::copy(entry.path(), output_path).expect("Unable to copy file");
            } else {
                std::fs::copy(entry.path(), output_dir.as_ref().join(entry.file_name()))
                    .expect("Unable to copy file");
            }
        }
    }
}

pub struct CopyGlobOptions {
    root: PathBuf,
    exclude: GlobSet,
}

pub struct CopyGlobOptionsBuilder {
    root: PathBuf,
    exclude: GlobSetBuilder,
}

impl CopyGlobOptionsBuilder {
    pub fn new() -> Self {
        let root = get_root_path();
        let mut exclude = GlobSet::builder();

        exclude.add(Glob::new("**/target/*").unwrap());

        Self { root, exclude }
    }
}

impl CopyGlobOptionsBuilder {
    pub fn set_root_path(mut self, root: impl AsRef<Path>) -> Self {
        self.root = root.as_ref().to_path_buf();
        self
    }

    pub fn add_exclude(mut self, glob: impl AsRef<str>) -> Self {
        self.exclude.add(Glob::new(glob.as_ref()).unwrap());
        self
    }

    pub fn build(self) -> CopyGlobOptions {
        CopyGlobOptions {
            root: self.root,
            exclude: self.exclude.build().unwrap(),
        }
    }
}

pub fn new_glob(glob: impl AsRef<str>) -> GlobMatcher {
    Glob::new(glob.as_ref())
        .expect("Wrong pattern")
        .compile_matcher()
}

pub fn get_target_folder() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");

    #[cfg(debug_assertions)]
    return path.join("debug");

    #[cfg(not(debug_assertions))]
    return path.join("release");
}

pub fn get_root_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn get_str(path: &Path) -> &str {
    path.to_str().expect("Unable to convert path to str")
}
