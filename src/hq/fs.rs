use std::fs;
use util::ResultBox;
use ui::{Style, Formatted};

def_error! {
    Internal: "internal error",
}

#[derive(Debug, PartialEq)]
pub enum EntryType {
    File,
    Directory,
}

#[derive(Debug)]
pub struct Entry {
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

    /// Return if it is a directory.
    pub fn is_dir(&self) -> bool {
        self.file_type == EntryType::Directory
    }
}

impl Filesys {
    /// Initialze.
    pub fn new() -> ResultBox<Filesys> {
        let mut fs = Filesys {
            path: String::new(),
            files: vec![],
        };
        fs.update(".")?;
        Ok(fs)
    }

    /// Refresh the files.
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
            self.files
                .push(Entry::new(entry
                                     .path()
                                     .file_name()
                                     .ok_or(Error::Internal)?
                                     .to_str()
                                     .ok_or(Error::Internal)?,
                                 entry_type));
        }
        Ok(())
    }

    pub fn render(&self) -> Vec<Formatted> {
        let mut res = vec![];
        for entry in &self.files {
            let style = if entry.is_dir() {
                Style::Directory
            } else {
                Style::File
            };
            res.push(Formatted::new().push(style, &entry.name));
        }
        res
    }
}
