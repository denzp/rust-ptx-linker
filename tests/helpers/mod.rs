macro_rules! assert_files_eq {
    ($lhs: expr, $rhs: expr) => {
        use std::io::{BufReader, Read};
        use std::fs::File;
        use difference::{Changeset, Difference};

        let mut lhs_file = BufReader::new(File::open(&$lhs).unwrap());
        let mut rhs_file = BufReader::new(File::open(&$rhs).unwrap());

        let mut lhs_contents = String::new();
        let mut rhs_contents = String::new();
        lhs_file.read_to_string(&mut lhs_contents).unwrap();
        rhs_file.read_to_string(&mut rhs_contents).unwrap();

        // {
        //     use std::io::{BufWriter, Write};
        //     let mut rhs_writer = BufWriter::new(File::create(&$rhs).unwrap());

        //     rhs_writer.write_all(lhs_contents.as_bytes()).unwrap();
        // }

        let Changeset { diffs, .. } = Changeset::new(&lhs_contents, &rhs_contents, "\n");

        if lhs_contents != rhs_contents {
            for diff in diffs {
                match diff {
                    Difference::Same(ref chunk) => {
                        for line in chunk.split("\n").take(2) {
                            eprintln!("\x1b[37m{}\x1b[0m", line);
                        }

                        let lines = chunk.split("\n").count();

                        if lines > 2 {
                            eprintln!("\x1b[90m----------\x1b[0m");
                            for line in chunk.split("\n").skip(lines - 2) {
                                eprintln!("\x1b[37m{}\x1b[0m", line);
                            }
                        }
                    }
                    Difference::Add(ref chunk) => for line in chunk.split("\n") {
                        eprintln!(
                            "\x1b[92m{}\x1b[0m",
                            if line == "" { "(empty line)" } else { line }
                        );
                    },
                    Difference::Rem(ref chunk) => for line in chunk.split("\n") {
                        eprintln!(
                            "\x1b[91m{}\x1b[0m",
                            if line == "" { "(empty line)" } else { line }
                        );
                    },
                };
            }
        }

        assert!(lhs_contents == rhs_contents, "files are different");
    };
}
