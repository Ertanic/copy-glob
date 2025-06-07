use copy_glob::{
    CopyGlobOptionsBuilder, copy_glob, copy_glob_with, get_root_path, get_target_folder,
};
use std::path::PathBuf;

#[test]
fn test_copy_files() {
    let expected_output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug")
        .join("copies1");
    let actual_output = get_target_folder().join("copies1");
    assert_eq!(expected_output, actual_output);

    copy_glob("**/for_copy/*", &actual_output);

    let tests_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("for_copy");
    let output_dir = expected_output.join("tests").join("for_copy");
    let files = &[
        "ok.js",
        "not_empty",
        "not_empty\\rust\\ruru.rs",
        "not_empty\\holo.yml",
        "empty",
    ];

    for file in files {
        let copy_path = output_dir.join(file);
        let original_path = tests_folder.join(file);
        let meta = copy_path.metadata().unwrap();

        if meta.is_file() {
            let original_content = std::fs::read_to_string(original_path).unwrap();
            let copy_content = std::fs::read_to_string(copy_path).unwrap();

            assert_eq!(original_content, copy_content);
        } else if meta.is_dir() {
            let original_meta = std::fs::metadata(&original_path).unwrap();
            assert!(original_meta.is_dir());

            let original_children = std::fs::read_dir(&original_path).unwrap();
            let copy_children = std::fs::read_dir(copy_path).unwrap();
            assert_eq!(original_children.count(), copy_children.count());
        } else {
            unimplemented!("Unsupported")
        }
    }
}

#[test]
fn test_copy_files_with_options() {
    let expected_output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug")
        .join("copies2");
    let actual_output = get_target_folder().join("copies2");
    assert_eq!(expected_output, actual_output);

    let root = get_root_path().join("tests").join("for_copy");
    let options = CopyGlobOptionsBuilder::new().set_root_path(root).build();
    copy_glob_with("*.rs", &actual_output, &options);

    let tests_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("for_copy");
    let output_dir = expected_output;
    let files = &[
        &["ruru.rs", "not_empty\\rust\\ruru.rs"],
        &["rara.rs", "rara.rs"],
    ];

    for file in files {
        let copy_path = output_dir.join(file[0]);
        let original_path = tests_folder.join(file[1]);
        let meta = copy_path.metadata().unwrap();

        if meta.is_file() {
            let original_content = std::fs::read_to_string(original_path).unwrap();
            let copy_content = std::fs::read_to_string(copy_path).unwrap();

            assert_eq!(original_content, copy_content);
        } else if meta.is_dir() {
            let original_meta = std::fs::metadata(&original_path).unwrap();
            assert!(original_meta.is_dir());

            let original_children = std::fs::read_dir(&original_path).unwrap();
            let copy_children = std::fs::read_dir(copy_path).unwrap();
            assert_eq!(original_children.count(), copy_children.count());
        } else {
            unimplemented!("Unsupported")
        }
    }
}
