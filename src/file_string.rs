use regex::Regex;
use crate::SortError;

pub fn is_pre_group(string: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"isa\s=\sPBXGroup;").unwrap();
    }
    RE.is_match(string)
}

pub fn has_block_name(string: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)/\*\s+.+\s+\*/\s=\s\{$").unwrap();
    }
    RE.is_match(string)
}

pub fn is_start_parent(string: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"children\s=\s\(").unwrap();
    }
    RE.is_match(string)
}

pub fn is_end_parent(string: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\);").unwrap();
    }
    RE.is_match(string)
}

pub fn children_name(string: &str) -> Result<&str, SortError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)/\*\s+(?P<name>.+)\s+\*/,$").unwrap();
    }
    let error = SortError::ChildrenNameParserError;
    match RE.captures(string) {
        Some(cap) => cap.name("name").map(|name|name.as_str()).ok_or(error),
        None => Err(error)
    }
}

pub fn is_start_files(string: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"/\*\sBegin\sPBXFileReference\ssection\s\*/").unwrap();
    }
    RE.is_match(string)
}

pub fn is_end_files(string: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"/\*\sEnd\sPBXFileReference\ssection\s\*/").unwrap();
    }
    RE.is_match(string)
}

pub fn has_file_id(string: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)^\s*[^\s]+\s+/\*\s+").unwrap();
    }
    RE.is_match(string)
}

pub fn file_id(string: &str) ->  Result<&str, SortError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)^\s*(?P<name>[^\s]+)\s+/\*\s+").unwrap();
    }
    let error = SortError::FileIdParserError;
    match RE.captures(string) {
        Some(cap) => cap.name("name").map(|name|name.as_str()).ok_or(error),
        None => Err(error)
    }
}

#[cfg(test)]
mod file_string_tests {
    use crate::file_string::*;

    #[test]
    fn should_be_pre_group() {
        assert_eq!(is_pre_group("isa = PBXGroup;"), true);
        assert_eq!(is_pre_group("some"), false);
    }

    #[test]
    fn should_parse_block_name() {
        assert_eq!(has_block_name("DC3EDF8121556470004B337E /* Auth */ = {"), true);
        assert_eq!(has_block_name("DC3EDF8121556470004B337E /* Auth = {"), false);
    }

    #[test]
    fn should_be_start_parent() {
        assert_eq!(is_start_parent("children = ("), true);
    }

    #[test]
    fn should_be_end_parent() {
        assert_eq!(is_end_parent(");"), true);
    }

    #[test]
    fn should_parse_children_name() {
        let string = "DC3EDF8821556612004B337E /* MainViewController.swift */,";
        assert_eq!(children_name(string).unwrap(), "MainViewController.swift");
    }

    #[test]
    fn should_be_start_files() {
        assert_eq!(is_start_files("/* Begin PBXFileReference section */"), true);
    }

    #[test]
    fn should_be_end_files() {
        assert_eq!(is_end_files( "/* End PBXFileReference section */"), true);
    }

    #[test]
    fn should_be_file_id() {
        let string = r#"DC6DBBF9215F677A004742CA /* File */ = {isa = PBXFileReference; lastKnownFileType = text; path = File; sourceTree = "<group>"; }; */"#;
        assert_eq!(has_file_id(string), true);
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn should_parse_file_id() {
        let string = r#"DC6DBBF9215F677A004742CA /* File */ = {isa = PBXFileReference; lastKnownFileType = text; path = File; sourceTree = "<group>"; }; */"#;
        assert_eq!(file_id(string).unwrap(),"DC6DBBF9215F677A004742CA");
    }

}