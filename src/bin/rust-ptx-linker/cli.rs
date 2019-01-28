use std::path::Path;

use clap::{App, Arg, ArgMatches};
use ptx_linker::session::{OptLevel, Output, Session};

pub fn current_session() -> Session {
    parse_session(get_app().get_matches())
}

fn get_app() -> App<'static, 'static> {
    App::new("rust-ptx-linker")
        .version(crate_version!())
        .author(crate_authors!())
        .about("CUDA PTX linker for Rust crates.")
        .args(&[
            {
                Arg::with_name("bitcode")
                    .long("bitcode")
                    .help("Input LLVM bitcode file")
                    .display_order(0)
                    .takes_value(true)
                    .multiple(true)
                    .number_of_values(1)
            },
            {
                Arg::with_name("rlib")
                    .long("rlib")
                    .help("Input Rust rlib archive")
                    .display_order(1)
                    .takes_value(true)
                    .multiple(true)
                    .number_of_values(1)
            },
            {
                Arg::with_name("output")
                    .short("o")
                    .help("Output PTX assembly path")
                    .display_order(2)
                    .takes_value(true)
                    .value_name("PATH.ptx")
            },
            {
                Arg::with_name("input_dir")
                    .short("L")
                    .help("Input files directory")
                    .takes_value(true)
                    .value_name("PATH")
                    .multiple(true)
                    .number_of_values(1)
            },
            {
                Arg::with_name("optimisation")
                    .short("O")
                    .help("Optimisation level")
                    .takes_value(true)
                    .possible_values(&["lto"])
                    .value_name("level")
            },
            {
                Arg::with_name("debug")
                    .long("debug")
                    .help("Emit debug info")
            },
            {
                Arg::with_name("arch")
                    .short("a")
                    .long("arch")
                    .help("Target CUDA architectures")
                    .takes_value(true)
                    .multiple(true)
                    .number_of_values(1)
                    .use_delimiter(true)
            },
            {
                Arg::with_name("emit")
                    .short("e")
                    .long("emit")
                    .help("Output kind")
                    .takes_value(true)
                    .possible_values(&["asm", "ptx", "llvm-ir", "llvm-bc"])
                    .default_value("asm")
                    .multiple(true)
                    .number_of_values(1)
                    .use_delimiter(true)
            },
        ])
}

fn parse_session(matches: ArgMatches<'static>) -> Session {
    let mut session = Session::default();

    if let Some(inputs) = matches.values_of("bitcode") {
        for input in inputs {
            session.link_bitcode(Path::new(input));
        }
    }

    if let Some(inputs) = matches.values_of("rlib") {
        for input in inputs {
            session.link_rlib(Path::new(input));
        }
    }

    if let Some(output) = matches.value_of("output") {
        session.set_output(Path::new(output));
    }

    if matches.is_present("debug") {
        session.set_debug_info(true);
    }

    match matches.value_of("optimisation") {
        Some("lto") => session.set_opt_level(OptLevel::LTO),
        None => session.set_opt_level(OptLevel::None),

        Some(_) => {
            warn!("Not supported optimisation level! Ignoring...");
        }
    };

    if let Some(outputs) = matches.values_of("emit") {
        for output in outputs {
            session.add_output_type(match output {
                "llvm-ir" => Output::IntermediateRepresentation,
                "llvm-bc" => Output::Bitcode,

                // CLI arg has `possible_values` anyway
                _ => Output::PTXAssembly,
            });
        }
    }

    if let Some(archs) = matches.values_of("arch") {
        for arch in archs {
            session.add_output_arch(arch);
        }
    }

    session
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use ptx_linker::session::Output;

    #[test]
    fn it_should_parse_bitcode_inputs() {
        let matches = get_app().get_matches_from_safe(vec![
            "rust-ptx-linker",
            "-L",
            "/rustlib/lib",
            "--bitcode",
            "/kernel/target/debug/deps/kernel.0.o",
            "--bitcode",
            "/kernel/target/debug/deps/kernel.crate.metadata.o",
            "--bitcode",
            "/kernel/target/debug/deps/kernel.1.o",
        ]);

        assert_eq!(
            parse_session(matches.unwrap()),
            Session {
                emit: vec![Output::PTXAssembly],
                achitectures: vec![],

                opt_level: OptLevel::None,
                debug_info: false,

                output: None,
                include_rlibs: vec![],

                include_bitcode_modules: vec![
                    PathBuf::from("/kernel/target/debug/deps/kernel.0.o"),
                    PathBuf::from("/kernel/target/debug/deps/kernel.1.o"),
                ],
            }
        );
    }

    #[test]
    fn it_should_parse_rlib_inputs() {
        let matches = get_app().get_matches_from_safe(vec![
            "rust-ptx-linker",
            "-L",
            "/rustlib/lib",
            "--rlib",
            "/kernel/target/debug/deps/kernel.0.rlib",
            "--rlib",
            "/kernel/target/debug/deps/kernel.1.rlib",
        ]);

        assert_eq!(
            parse_session(matches.unwrap()),
            Session {
                emit: vec![Output::PTXAssembly],
                achitectures: vec![],

                opt_level: OptLevel::None,
                debug_info: false,

                output: None,
                include_bitcode_modules: vec![],

                include_rlibs: vec![
                    PathBuf::from("/kernel/target/debug/deps/kernel.0.rlib"),
                    PathBuf::from("/kernel/target/debug/deps/kernel.1.rlib"),
                ],
            }
        );
    }

    #[test]
    fn it_should_parse_output() {
        let matches = get_app().get_matches_from_safe(vec![
            "rust-ptx-linker",
            "-o",
            "/kernel/target/debug/deps/kernel.ptx",
        ]);

        assert_eq!(
            parse_session(matches.unwrap()),
            Session {
                emit: vec![Output::PTXAssembly],
                achitectures: vec![],

                opt_level: OptLevel::None,
                debug_info: false,

                output: Some(PathBuf::from("/kernel/target/debug/deps/kernel.ptx")),
                include_bitcode_modules: vec![],
                include_rlibs: vec![],
            }
        );
    }

    #[test]
    fn it_should_parse_debug_flag() {
        let matches = get_app().get_matches_from_safe(vec!["rust-ptx-linker", "--debug"]);

        assert_eq!(
            parse_session(matches.unwrap()),
            Session {
                emit: vec![Output::PTXAssembly],
                achitectures: vec![],

                opt_level: OptLevel::None,
                debug_info: true,

                output: None,
                include_bitcode_modules: vec![],
                include_rlibs: vec![],
            }
        );
    }

    #[test]
    fn it_should_parse_optimisations() {
        assert_eq!(
            parse_session(
                get_app()
                    .get_matches_from_safe(vec!["rust-ptx-linker", "-Olto"])
                    .unwrap()
            ),
            Session {
                emit: vec![Output::PTXAssembly],
                achitectures: vec![],

                opt_level: OptLevel::LTO,
                debug_info: false,

                output: None,
                include_bitcode_modules: vec![],
                include_rlibs: vec![],
            }
        );
    }

    #[test]
    fn it_should_parse_emit() {
        assert_eq!(
            parse_session(
                get_app()
                    .get_matches_from_safe(vec!["rust-ptx-linker", "--emit", "asm"])
                    .unwrap()
            ),
            Session {
                emit: vec![Output::PTXAssembly],
                achitectures: vec![],

                opt_level: OptLevel::None,
                debug_info: false,

                output: None,
                include_bitcode_modules: vec![],
                include_rlibs: vec![],
            }
        );

        assert_eq!(
            parse_session(
                get_app()
                    .get_matches_from_safe(vec!["rust-ptx-linker", "--emit", "ptx"])
                    .unwrap()
            ),
            Session {
                emit: vec![Output::PTXAssembly],
                achitectures: vec![],

                opt_level: OptLevel::None,
                debug_info: false,

                output: None,
                include_bitcode_modules: vec![],
                include_rlibs: vec![],
            }
        );

        assert_eq!(
            parse_session(
                get_app()
                    .get_matches_from_safe(vec!["rust-ptx-linker", "--emit", "llvm-ir"])
                    .unwrap()
            ),
            Session {
                emit: vec![Output::IntermediateRepresentation],
                achitectures: vec![],

                opt_level: OptLevel::None,
                debug_info: false,

                output: None,
                include_bitcode_modules: vec![],
                include_rlibs: vec![],
            }
        );

        assert_eq!(
            parse_session(
                get_app()
                    .get_matches_from_safe(vec!["rust-ptx-linker", "--emit", "llvm-bc"])
                    .unwrap()
            ),
            Session {
                emit: vec![Output::Bitcode],
                achitectures: vec![],

                opt_level: OptLevel::None,
                debug_info: false,

                output: None,
                include_bitcode_modules: vec![],
                include_rlibs: vec![],
            }
        );

        assert_eq!(
            parse_session(
                get_app()
                    .get_matches_from_safe(vec![
                        "rust-ptx-linker",
                        "--emit",
                        "asm",
                        "--emit",
                        "llvm-bc"
                    ])
                    .unwrap()
            ),
            Session {
                emit: vec![Output::PTXAssembly, Output::Bitcode],
                achitectures: vec![],

                opt_level: OptLevel::None,
                debug_info: false,

                output: None,
                include_bitcode_modules: vec![],
                include_rlibs: vec![],
            }
        );

        assert_eq!(
            parse_session(
                get_app()
                    .get_matches_from_safe(vec!["rust-ptx-linker", "--emit", "asm,llvm-bc"])
                    .unwrap()
            ),
            Session {
                emit: vec![Output::PTXAssembly, Output::Bitcode],
                achitectures: vec![],

                opt_level: OptLevel::None,
                debug_info: false,

                output: None,
                include_bitcode_modules: vec![],
                include_rlibs: vec![],
            }
        );
    }

    #[test]
    fn it_should_parse_arch() {
        assert_eq!(
            parse_session(
                get_app()
                    .get_matches_from_safe(vec!["rust-ptx-linker", "--arch", "sm_60"])
                    .unwrap()
            ),
            Session {
                emit: vec![Output::PTXAssembly],
                achitectures: vec![String::from("sm_60")],

                opt_level: OptLevel::None,
                debug_info: false,

                output: None,
                include_bitcode_modules: vec![],
                include_rlibs: vec![],
            }
        );

        assert_eq!(
            parse_session(
                get_app()
                    .get_matches_from_safe(vec![
                        "rust-ptx-linker",
                        "--arch",
                        "sm_50",
                        "--arch",
                        "sm_60"
                    ])
                    .unwrap()
            ),
            Session {
                emit: vec![Output::PTXAssembly],
                achitectures: vec![String::from("sm_50"), String::from("sm_60")],

                opt_level: OptLevel::None,
                debug_info: false,

                output: None,
                include_bitcode_modules: vec![],
                include_rlibs: vec![],
            }
        );

        assert_eq!(
            parse_session(
                get_app()
                    .get_matches_from_safe(vec!["rust-ptx-linker", "--arch", "sm_50,sm_60"])
                    .unwrap()
            ),
            Session {
                emit: vec![Output::PTXAssembly],
                achitectures: vec![String::from("sm_50"), String::from("sm_60")],

                opt_level: OptLevel::None,
                debug_info: false,

                output: None,
                include_bitcode_modules: vec![],
                include_rlibs: vec![],
            }
        );
    }
}
