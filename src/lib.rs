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
