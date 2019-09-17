#[macro_use] extern crate lazy_static;

pub mod file_string;

use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, LineWriter, Write};
use std::collections::HashSet;
use file_string::*;
use std::fs;
use std::path::{PathBuf, Path};

pub fn run(pbxproj_path: &String) -> Result<(), SortError> {
    let pbxproj_path = make_path(pbxproj_path);

    let pbxproj = File::open(&pbxproj_path).map_err(|_|SortError::OpenFileError)?;
    let origin_file_strings = BufReader::new(&pbxproj);

    let mut new_file_strings: Vec<String> = Vec::new();
    let mut current_group_strings: Vec<Group> = Vec::new();

    let mut project_files: HashSet<String> = HashSet::new();
    let mut read_mode = ReadMode::Normal;
    let mut prev_string = String::new();

    for string in origin_file_strings.lines() {
        let string = string.map_err(|_|SortError::ReadLineError)?;

        match read_mode {
            ReadMode::Files if has_file_id(&string) => { project_files.insert(file_id(&string)?.to_string()); },
            ReadMode::PreGroup if is_start_parent(&string) => current_group_strings.clear(),
            ReadMode::Group if is_end_parent(&string) => {
                current_group_strings.sort_by( |lhs, rhs| lhs.name.cmp(&rhs.name));
                current_group_strings.iter().filter(|group| !project_files.contains(&group.id)).for_each(|group| new_file_strings.push(group.string.clone()));
                current_group_strings.iter().filter(|group| project_files.contains(&group.id)).for_each(|group| new_file_strings.push(group.string.clone()));
                current_group_strings.clear();
            }
            _ => {}
        }

        read_mode = reduce_read_mode(&read_mode, &string, &prev_string);
        prev_string = string.clone();

        match (read_mode, children_name(&string), file_id(&string)) {
            (ReadMode::Group, Ok(name), Ok(id)) => {
                let group = Group { name: name.to_string(), id: id.to_string(), string };
                current_group_strings.push(group)
            },
            _ => new_file_strings.push(string)
        }

    }

    write_strings(&pbxproj_path, &mut new_file_strings);

    Ok(())
}

fn write_strings(pbxproj_path: &PathBuf, new_file_strings: &mut Vec<String>) -> Result<(), SortError> {
    fs::remove_file(&pbxproj_path).map_err(|_| SortError::WriteToFileError)?;
    let pbxproj = OpenOptions::new().create(true).write(true).open(pbxproj_path).map_err(|_| SortError::OpenFileError)?;
    let mut file = LineWriter::new(pbxproj);
    for string in new_file_strings.iter() {
        file.write_all(format!("{}\n", string).as_bytes()).map_err(|_| SortError::WriteToFileError)?;
    }
    Ok(())
}

fn make_path(path: &str) -> PathBuf {
    let path = Path::new(path);
    if path.is_dir() {
        path.join("project.pbxproj")
    } else {
        path.to_path_buf()
    }
}

fn reduce_read_mode(mode: &ReadMode, string: &str, prev_string: &str) -> ReadMode {
    if is_start_files(string) {
        return ReadMode::Files;
    } else if is_end_files(string) {
        return ReadMode::Normal;
    } else if is_pre_group(string) && has_block_name(prev_string) {
        return ReadMode::PreGroup;
    } else if is_pre_group(string) && !has_block_name(prev_string) {
        return mode.clone();
    } else if mode == &ReadMode::PreGroup && is_start_parent(&string) {
        return ReadMode::Group;
    } else if mode == &ReadMode::Group && is_end_parent(&string) {
        return ReadMode::Normal;
    } else {
        return mode.clone();
    }
}

struct Group {
    name: String,
    id: String,
    string: String
}

#[derive(Clone, Copy, PartialEq)]
enum ReadMode {
    Files,
    Normal,
    PreGroup,
    Group,
}

#[derive(Debug)]
pub enum SortError {
    ChildrenNameParserError,
    FileIdParserError,
    OpenFileError,
    ReadLineError,
    WriteToFileError
}