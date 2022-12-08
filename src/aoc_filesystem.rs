use std::collections::HashMap;
use crate::{CdParam, Command, DirectoryMember};

#[derive(Debug)]
pub struct FilesystemBuilder {
    tree_root: FilesystemObject
}

#[derive(Debug)]
pub enum FilesystemObject {
    File(usize),
    Folder(HashMap<String, FilesystemObject>)
}

impl FilesystemObject {
    fn get_folder(&mut self, path: &[String]) -> Option<&mut FilesystemObject> {
        let mut root = self;
        for p in path {
            root = match root {
                FilesystemObject::File(_) => return None,
                FilesystemObject::Folder(c) => {
                    c.get_mut(p)?
                }
            };
        };
        Some(root)
    }

}

impl FilesystemBuilder {
    pub(crate) fn new() -> Self {
        FilesystemBuilder{
            tree_root: FilesystemObject::Folder(HashMap::new())
        }
    }
    pub fn build(self) -> FilesystemObject {
        self.tree_root
    }

    pub(crate) fn execute(mut self, commands: &[crate::Command]) -> Self {
        let mut current_dir = Vec::new();
        commands.iter().for_each(|c| match c {
            Command::Cd(target) => {
                match target {
                    CdParam::In(to) => current_dir.push(to.clone()),
                    CdParam::Out => {current_dir.pop();}
                    CdParam::Root => current_dir.clear()
                }
            }
            Command::Ls(content) => {
                match self.tree_root.get_folder(&current_dir).unwrap() {
                    FilesystemObject::File(_) => panic!("Expecting folder"),
                    FilesystemObject::Folder(d) => {
                        for c in content {
                            match c {
                                DirectoryMember::File(f, s) => {
                                    d.insert(f.clone(), FilesystemObject::File(*s))
                                }
                                DirectoryMember::Folder(f) => {
                                    d.insert(f.clone(), FilesystemObject::Folder(HashMap::new()))
                                }
                            };
                        }
                    }
                }
            }
        }
        );
        self
    }
}
