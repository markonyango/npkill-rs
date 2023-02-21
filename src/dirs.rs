use bytesize::ByteSize;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub struct NodeModuleEntry {
    pub dir_entry: DirEntry,
    pub size: String,
}

pub fn get_dirs(path: &str) -> Vec<NodeModuleEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_entry(is_node_modules_root)
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().ends_with("node_modules"))
        .map(|entry| NodeModuleEntry {
            dir_entry: entry.clone(),
            size: get_dir_size(&entry),
        })
        .collect()
}

pub fn get_dir_size(path: &DirEntry) -> String {
    let size = WalkDir::new(path.path())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .fold(0, |size, item| {
            size + item.metadata().unwrap().len() as usize
        });

    ByteSize(size as u64).to_string()
}

fn is_node_modules_root(entry: &DirEntry) -> bool {
    entry.path().is_dir()
        && entry
            .path()
            .to_str()
            .map(|s| s.matches("node_modules").count() < 2)
            .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::get_dirs;

    #[test]
    fn only_root_node_modules() {
        let dirs = get_dirs(".");
        let len = dirs.len();
        assert_eq!(len, 1)
    }
}
