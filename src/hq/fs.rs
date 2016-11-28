use std::path::Path;
use std::fs;
use util::ResultBox;

def_error! {
    Internal: "internal error",
}

#[derive(Debug)]
enum EntryType {
    File,
    Directory,
}

#[derive(Debug)]
struct Entry {
    name: String,
    file_type: EntryType,
}

#[derive(Debug)]
pub struct Filesys {
    path: String,
    files: Vec<Entry>,
}

impl Entry {
    pub fn new(name: &str, file_type: EntryType) -> Entry {
        Entry {
            name: String::from(name),
            file_type: file_type,
        }
    }
}

impl Filesys {
    pub fn new() -> Filesys {
        let mut fs = Filesys {
            path: String::new(),
            files: vec![],
        };
        fs.update(".");
        fs
    }

    fn update(&mut self, path: &str) -> ResultBox<()> {
        self.path = String::from(path);
        self.files.clear();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_type = if fs::metadata(entry.path())?.is_dir() {
                EntryType::Directory
            } else {
                EntryType::File
            };
            self.files.push(Entry::new(entry.path()
                                       .file_name()
                                       .ok_or(Error::Internal)?
                                       .to_str()
                                       .ok_or(Error::Internal)?, entry_type));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let fs = Filesys::new();
    }
}
