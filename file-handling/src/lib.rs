use regex::Regex;
use std::fs;
use std::io;

enum Kind {
    Dir,
    File,
}

fn retrive_name_vec(path: String, kind: Kind) -> io::Result<Vec<String>> {
    let mut names = vec![];

    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let flag = match kind {
            Kind::Dir => entry.file_type()?.is_dir(),
            Kind::File => entry.file_type()?.is_file(),
        };
        if flag {
            let name = entry.file_name().into_string().unwrap();
            names.push(name);
        }
    }

    Ok(names)
}

pub fn retrive_folder_name_vec(path: String) -> io::Result<Vec<String>> {
    retrive_name_vec(path, Kind::Dir)
}

pub fn retrive_file_name_vec(path: String) -> io::Result<Vec<String>> {
    retrive_name_vec(path, Kind::File)
}

pub fn retrive_name_vec_with_pattern_match(names: Vec<String>, pattern: &str) -> Vec<String> {
    //! regular expression
    let regex = Regex::new(pattern).unwrap();
    names
        .into_iter()
        .filter(|name| regex.is_match(name.as_str()))
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retrive_folder_names() {
        let path = "./test-data".to_string();
        let names = retrive_folder_name_vec(path);
        if let Ok(names) = names {
            assert_eq!(vec!["0001", "0002", "0003", "hoge"], names);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_retrive_file_names() {
        let path = ".".to_string();
        let names = retrive_file_name_vec(path);
        if let Ok(names) = names {
            assert_eq!(vec!["Cargo.toml"], names);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_retrive_names_with_pattern_match() {
        let path = "./test-data".to_string();
        let names = retrive_folder_name_vec(path);
        if let Ok(names) = names {
            let regex = r"\d{4}";
            let names = retrive_name_vec_with_pattern_match(names, regex);
            assert_eq!(vec!["0001", "0002", "0003"], names);
        } else {
            assert!(false);
        }
    }
}
