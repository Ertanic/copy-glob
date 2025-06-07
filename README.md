# About it

**copy-glob** is a small utility to copy files to an output directory using glob syntax. Inspired by [copy_to_output](https://crates.io/crates/copy_to_output).

# How to use

Add a crate to the build dependencies in `Cargo.toml`.

```toml
[build-dependencies]
copy-glob = "0.1"
```

Create `build.rs` and add the following to it:

```rust
use copy_glob::{get_target_folder, copy_glob};

fn main() {
    let output = get_target_folder().join("copies"); // target/{debug,release}/copies
    copy_glob("**/for_copy/*", output);
}
```

This will copy all the files in the directory, preserving the structure, since the copying starts from the root of the project (where the `Cargo.toml` file is located).

To localize the file search somehow, you can use the following code:

```rust
use copy_glob::{get_target_folder, get_root_path, CopyGlobOptionsBuilder, copy_glob_with};

fn main() {
    let output = get_target_folder().join("copies");
    let root = get_root_path().join("for_copy"); // same level as Cargo.toml
    let options = CopyGlobOptionsBuilder::new().set_root_path(root).build();
    copy_glob_with("*.toml", output, &options);
}
```

This will copy all files with the `.toml` extension in the `for_copy` folder without preserving the structure.

In addition, you can add exceptions from where files will be copied. By default, the `target` directory hangs in the excludes.

```rust
let options = CopyGlobOptionsBuilder::new().add_exclude("**/some_pattern/*").build();
```