use std::collections::HashMap;
use std::fs;
use aoc_filesystem::{FilesystemBuilder, FilesystemObject};
use crate::command_parsing::{CdParam, Command, DirectoryMember};

mod command_parsing;
mod aoc_filesystem;

const TOTAL_DISK_SPACE:usize = 70_000_000;
const DISK_SPACE_REQUIRED: usize = 30_000_000;

fn main() {
    let file = fs::read_to_string("./actual_input.txt").unwrap();
    let commands = command_parsing::parse(&file);
    println!("{:#?}", commands);

    let fs = FilesystemBuilder::new().execute(&commands).build();
    println!("{:#?}", fs);

    let mut directory_sizes = HashMap::new();
    let used_disk_space = get_directory_sizes(&fs, &mut vec!["".to_string()], &mut directory_sizes);
    println!("PART 1: Sum of directories of size <= 100000: {}", directory_sizes.values().filter(|s|**s <= 100_000).sum::<usize>());
    let required_to_free = DISK_SPACE_REQUIRED.saturating_sub(TOTAL_DISK_SPACE - used_disk_space);
    println!("Total {} Used {} Required {} Space needed {}", TOTAL_DISK_SPACE, used_disk_space, DISK_SPACE_REQUIRED, required_to_free);
    print!("PART 2: ");
    if required_to_free > 0 {
        let to_free = directory_sizes.iter().filter(|(_, s)| **s >= required_to_free).min_by(|a, b|a.1.cmp(b.1)).unwrap();
        println!("Delete directory {} with size {}", to_free.0, to_free.1);
    }else {
        println!("There is enough disk space available")
    }
}

fn get_directory_sizes(root: &FilesystemObject, current_path: &mut Vec<String>, results: &mut HashMap<String, usize>) -> usize {
    match root {
        FilesystemObject::File(s) => *s,
        FilesystemObject::Folder(children) => {
            let mut directory_size = 0;
            for (k, c) in children {
                current_path.push(k.clone());
                directory_size += get_directory_sizes(c, current_path, results);
                current_path.pop();
            }

            results.insert(current_path.join("/"), directory_size);
            directory_size
        }
    }
}