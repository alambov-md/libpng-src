use libpng_src::source_path;

#[test]
fn test_source_path_is_dir() {
    let source_path = source_path();
    assert!(source_path.is_dir());
}

#[test]
fn test_source_path_contains_headers() {
    let source_path = source_path();

    assert!(source_path.join("png.h").is_file());
    assert!(source_path.join("pngconf.h").is_file());
}
