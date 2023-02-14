use regex::Regex;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

enum Kind {
    Dir,
    File,
}

fn retrive_name_vec(path: &Path, kind: Kind) -> io::Result<Vec<PathBuf>> {
    let mut names = vec![];

    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let flag = match kind {
            Kind::Dir => entry.file_type()?.is_dir(),
            Kind::File => entry.file_type()?.is_file(),
        };
        if flag {
            let name = PathBuf::from(entry.file_name());
            names.push(name.clone());
        }
    }

    Ok(names)
}

pub fn retrive_folder_name_vec(path: &Path) -> io::Result<Vec<PathBuf>> {
    retrive_name_vec(path, Kind::Dir)
}

pub fn retrive_file_name_vec(path: &Path) -> io::Result<Vec<PathBuf>> {
    retrive_name_vec(path, Kind::File)
}

pub fn retrive_name_vec_with_pattern_match(names: Vec<PathBuf>, pattern: &str) -> Vec<PathBuf> {
    //! regular expression
    let regex = Regex::new(pattern).unwrap();

    names
        .into_iter()
        .filter(|name| regex.is_match(name.file_name().unwrap().to_str().unwrap()))
        .collect::<Vec<PathBuf>>()
}

pub fn copy_files(path_vec: Vec<PathBuf>, dist: &Path) -> io::Result<()> {
    for path in path_vec {
        let to = dist.join(path.file_name().unwrap());
        fs::copy(path, to)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retrive_folder_names() {
        let ans = vec!["0001", "0002", "0003", "hoge"];
        let ans = ans
            .iter()
            .map(|&a| PathBuf::from(a))
            .collect::<Vec<PathBuf>>();

        let path = Path::new("./test-data");
        let names = retrive_folder_name_vec(path);
        if let Ok(names) = names {
            assert_eq!(ans, names);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_retrive_file_names() {
        let path = Path::new(".");
        let names = retrive_file_name_vec(path);
        if let Ok(names) = names {
            assert_eq!(vec![PathBuf::from("Cargo.toml")], names);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_retrive_names_with_pattern_match() {
        let ans = vec!["0001", "0002", "0003"];
        let ans = ans
            .iter()
            .map(|&a| PathBuf::from(a))
            .collect::<Vec<PathBuf>>();

        let path = Path::new("./test-data");
        let names = retrive_folder_name_vec(path);
        if let Ok(names) = names {
            let regex = r"\d{4}";
            let names = retrive_name_vec_with_pattern_match(names, regex);
            assert_eq!(ans, names);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_copy_files() {
        let ans = vec!["1.txt", "2.txt", "Cargo.toml"];
        let ans = ans
            .iter()
            .map(|&a| PathBuf::from(a))
            .collect::<Vec<PathBuf>>();

        let files = vec!["./test-data/1.txt", "./test-data/2.txt"];
        let path_vec = files
            .iter()
            .map(|&f| PathBuf::from(f))
            .collect::<Vec<PathBuf>>();

        path_vec.iter().for_each(|path| {
            fs::File::create(path).ok();
        });

        let dist = Path::new(".");
        copy_files(path_vec.clone(), dist).ok();

        let names = retrive_file_name_vec(Path::new(".")).ok().unwrap();
        assert_eq!(ans, names);

        // 後始末
        path_vec.iter().for_each(|path| {
            fs::remove_file(path).ok();
        });

        names
            .iter()
            .filter(|f| f.file_name().unwrap() != "Cargo.toml")
            .for_each(|f| {
                fs::remove_file(f).ok();
            })
    }
}
