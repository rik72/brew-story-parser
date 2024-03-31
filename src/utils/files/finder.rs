pub trait Finder {
    fn find_files(filename: &str, path: &String) -> Vec<String>;
}
