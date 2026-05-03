use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::Path;

pub mod cmd;
pub(crate) mod error;

pub const SKEL_DIR: &str = ".cache/ms0503-skel";
pub const SKEL_REPO: &str = "https://github.com/ms0503/skeletons";
pub const SKEL_REPO_REV: &str = "main";

pub struct VisitDir {
    children: Box<dyn Iterator<Item = VisitDir>>,
    root: Box<dyn Iterator<Item = io::Result<DirEntry>>>
}

impl VisitDir {
    pub fn new<P>(path: P) -> io::Result<Self>
    where
        P: AsRef<Path>
    {
        let children = Box::new(fs::read_dir(&path)?.filter_map(|e| {
            let e = e.ok()?;
            if e.file_type().ok()?.is_dir() {
                Some(VisitDir::new(e.path()).ok()?)
            } else {
                None
            }
        }));
        let root = Box::new(fs::read_dir(&path)?);
        Ok(Self {
            children,
            root
        })
    }

    pub fn entries(self) -> Box<dyn Iterator<Item = io::Result<DirEntry>>> {
        Box::new(
            self.root
                .chain(self.children.map(|s| s.entries()).flatten())
        )
    }
}

impl Iterator for VisitDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.root.next() {
            return Some(item);
        }
        if let Some(child) = self.children.next() {
            self.root = child.entries();
            return self.next();
        }
        None
    }
}
