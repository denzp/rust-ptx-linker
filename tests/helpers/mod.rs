macro_rules! get_build_dir {
    () => {
        current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_string_lossy()
    };
}

macro_rules! assert_file_contains {
    ($path:expr, $list:expr) => {{
        use std::fs::File;
        use std::io::{BufReader, Read};

        let mut contents = String::new();
        let mut file = BufReader::new(File::open(&$path).unwrap());

        file.read_to_string(&mut contents).unwrap();

        for item in &$list {
            assert!(
                contents.contains(item),
                format!("File {:?} do not contains {:?}", $path, item)
            );
        }
    }};
}

macro_rules! assert_file_not_contains {
    ($path:expr, $list:expr) => {{
        use std::fs::File;
        use std::io::{BufReader, Read};

        let mut contents = String::new();
        let mut file = BufReader::new(File::open(&$path).unwrap());

        file.read_to_string(&mut contents).unwrap();

        for item in &$list {
            assert!(
                !contents.contains(item),
                format!("File {:?} contains {:?}", $path, item)
            );
        }
    }};
}
