use std::path::{Path, PathBuf, Component,};
use std::env::current_dir;

/// Experimental utility for resolving the manifest path.
/// 
/// # Arguments
/// * `file`    - the current script file path (obtained with `file!()` macro);
/// * `path`    - the "path/to/manifest.json" string slice.
/// 
/// # Panics
/// This function might panic in various occasions:
/// - if `file` has no parent directory (e.g. "src/main.rs" parent would be "src");
/// - if it fails to find the `file` first segment somehow (e.g. "path/to/file" => "path");
/// - if the first file path [`Component`] is neither `RootDir` nor `Normal`;
/// - if final result fails to be canonicalized.
/// 
/// The last situation might occur if the final path doesn't really lead to
/// any existing file/directory. Or perhaps, because this function has a bug!
/// In this case, please open an issue at our GitHub repository!
/// 
/// # Example
/// ```plaintext
/// example/
/// |-- .dist/
/// |   |-- .vite/
/// |   |   |-- manifest.json
/// |-- src/
/// |   |-- main.rs
/// ```
/// ```ignore
/// 
/// // example/src/main.rs
/// let manifest_path = resolve_path(file!(), "../dist/.vite/manifest.json");
/// let mut vite_config = ViteConfig::new_with_defaults(&manifest_path);
/// 
/// println!("{manifest_path}");
/// // C:/totally/absolute/path/to/example/.dist/.vite/manifest.json
/// ```
/// 
/// [`Component`]: std::path::Component
pub fn resolve_path(file: &str, path: &str) -> String {
    let path: &Path = std::path::Path::new(path);
    let mut this_file_directory = Path::new(file)
        .parent()
        .expect("Could not get current file's directory.")
        .to_path_buf();
    
    #[allow(unused_mut)]
    let mut fp_fs: String; // file path first segment
    match this_file_directory.components().next() {
        Some(Component::Normal(segment)) => fp_fs = segment.to_string_lossy().to_string(),
        Some(Component::RootDir) => {
            let component = this_file_directory
                .components()
                .next()
                .expect(&format!(
                    "Failed to find first directory segment from path {}.",
                    this_file_directory.to_string_lossy())
                );

            match component {
                Component::Normal(segment) => fp_fs = segment.to_string_lossy().to_string(),
                _ => {
                    panic!(
                        "Failed to find first directory normal segment from path {}.",
                        this_file_directory.to_string_lossy()
                    )
                }
            }
                
        },
        _ => panic!("Unexpected kind of directory."),
    }

    let curr_dir = current_dir().unwrap();

    let paths_are_redundant = curr_dir.ends_with(&fp_fs) &&
        this_file_directory.starts_with(&fp_fs);
    
    // remove the first segment from this_file_directory so that it won't get doubled
    // on canonicalization
    if paths_are_redundant  {
        let mut new_path = PathBuf::new();
        let mut components = this_file_directory.components();

        if let Some(_) = components.next() {
            components.for_each(|component| new_path.push(component));
        }

        this_file_directory = new_path;
    }

    let joined_path = this_file_directory.join(path);
    let canonicalized = joined_path.canonicalize();
    match canonicalized {
        Err(err) => panic!("{}\n{}\n", err, joined_path.to_string_lossy()),
        Ok(path) => path.to_string_lossy().to_string()
    }
}

#[cfg(test)]
mod test {
    use std::io::Read;

    #[test]
    fn test_resolve_path() {
        let abs_path = "tests/dummy.txt";
        let rel_path = "../../tests/dummy.txt";
        let resolved_rel_path = super::resolve_path(file!(), &rel_path);

        let mut abs_file_contents = String::new();
        let mut rel_file_contents = String::new();

        let _ = std::fs::File::open(abs_path).unwrap().read_to_string(&mut abs_file_contents);
        let _ = std::fs::File::open(resolved_rel_path).unwrap().read_to_string(&mut rel_file_contents);

        assert_eq!(abs_file_contents, rel_file_contents);
    }
}
