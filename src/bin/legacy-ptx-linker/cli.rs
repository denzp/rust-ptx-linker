use std::path::Path;

use clap::{App, Arg, ArgMatches};
use ptx_linker::session::{OptLevel, Output, Session};

#[derive(Debug, PartialEq)]
pub enum CommandLineRequest {
    Link(Session),
    Print64BitTargetJson,
    Print32BitTargetJson,
}

pub fn get_cli_request() -> CommandLineRequest {
    CommandLineRequest::from(get_app().get_matches())
}

fn get_app() -> App<'static, 'static> {
    App::new("legacy-ptx-linker")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Performs linking of Rust NVPTX crates")
        .args(&[
            Arg::with_name("input")
                .help("Path to input rlib or bitcode")
                .multiple(true),
            Arg::with_name("output")
                .short("o")
                .help("Sets path for assembly output")
                .takes_value(true)
                .value_name("FILE.ptx"),
            Arg::with_name("optimise")
                .short("O")
                .help("Sets optimisation level")
                .takes_value(true)
                .value_name("level"),
            Arg::with_name("__ignored_lib_start")
                .long("whole-archive")
                .hidden(true)
                .multiple(true),
            Arg::with_name("__ignored_lib_end")
                .long("no-whole-archive")
                .hidden(true)
                .multiple(true),
            Arg::with_name("__ignored_start_group")
                .long("start-group")
                .hidden(true)
                .multiple(true),
            Arg::with_name("__ignored_end_group")
                .long("end-group")
                .hidden(true)
                .multiple(true),
            Arg::with_name("__ignored_compiler_prefix")
                .short("B")
                .takes_value(true)
                .hidden(true)
                .multiple(true)
                .number_of_values(1),
            Arg::with_name("__ignored_shared_hack")
                .short("s")
                .hidden(true)
                .multiple(true)
                .takes_value(true),
            Arg::with_name("__ignored_lib_path")
                .short("L")
                .takes_value(true)
                .hidden(true)
                .multiple(true)
                .number_of_values(1),
            Arg::with_name("__ignored_version_script")
                .long("version-script")
                .hidden(true)
                .multiple(true),
            Arg::with_name("__ignored_gc_sections")
                .long("gc-sections")
                .hidden(true)
                .multiple(true),
            Arg::with_name("arch")
                .long("arch")
                .short("a")
                .help("Target CUDA architectures")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1)
                .use_delimiter(true),
            Arg::with_name("emit")
                .long("emit")
                .short("e")
                .help("Output type")
                .takes_value(true)
                .possible_values(&["asm", "ptx", "llvm-ir", "llvm-bc"])
                .default_value("asm")
                .multiple(true)
                .number_of_values(1)
                .use_delimiter(true),
        ])
        .subcommand({
            App::new("print")
                .about("Prints NVPTX target definition JSON into stdout")
                .arg(
                    &Arg::with_name("TARGET")
                        .help("Specifies the target name")
                        .required(true)
                        .possible_values(&["nvptx64-nvidia-cuda", "nvptx-nvidia-cuda"]),
                )
        })
}

impl<'a> From<ArgMatches<'a>> for CommandLineRequest {
    fn from(matches: ArgMatches) -> CommandLineRequest {
        match matches.subcommand_name() {
            Some("print") => {
                let target = matches
                    .subcommand_matches("print")
                    .unwrap()
                    .value_of("TARGET");

                match target {
                    Some("nvptx64-nvidia-cuda") => CommandLineRequest::Print64BitTargetJson,
                    Some("nvptx-nvidia-cuda") => CommandLineRequest::Print32BitTargetJson,

                    other => {
                        unreachable!("Unknown target: {:?}", other);
                    }
                }
            }

            _ => {
                let mut session = Session::default();

                if let Some(inputs) = matches.values_of("input") {
                    for input in inputs {
                        if input.ends_with(".o") {
                            session.link_bitcode(Path::new(input));
                        } else if input.ends_with(".rlib") {
                            session.link_rlib(Path::new(input));
                        } else {
                            warn!("Can't recognise input type: {:?}", input);
                        }
                    }
                }

                if let Some(output) = matches.value_of("output") {
                    session.set_output(Path::new(output));
                }

                match matches.value_of("optimise") {
                    Some("0") | None => session.set_opt_level(OptLevel::None),
                    Some(_) => session.set_opt_level(OptLevel::Default),
                };

                if let Some(outputs) = matches.values_of("emit") {
                    for output in outputs {
                        session.add_output_type(match output {
                            "llvm-ir" => Output::IntermediateRepresentation,
                            "llvm-bc" => Output::Bitcode,
                            _ => Output::PTXAssembly,
                        });
                    }
                }

                if let Some(archs) = matches.values_of("arch") {
                    for arch in archs {
                        session.add_output_arch(arch);
                    }
                }

                CommandLineRequest::Link(session)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use ptx_linker::session::Output;

    #[test]
    fn it_should_parse_args() {
        let matches = get_app().get_matches_from_safe(vec![
            "legacy-ptx-linker",
            "-L",
            "/rustlib/lib",
            "/kernel/target/debug/deps/kernel.0.o",
            "/kernel/target/debug/deps/kernel.crate.metadata.o",
            "-o",
            "/kernel/target/debug/deps/libkernel.ptx",
            "-L",
            "/kernel/target/debug/deps",
            "-L",
            "/kernel/target/debug/deps",
            "-L",
            "~/rustlib/nvptx64-nvidia-cuda/lib",
            "-Bstatic",
            "--whole-archive",
            "/tmp/rustc.Ew934MzC8cj0/liblib-f0faab0dbaa9f7ef.rlib",
            "--no-whole-archive",
            "/tmp/rustc.Ew934MzC8cj0/libother-6b4931ba2f43f84b.rlib",
        ]);

        let expected_session = Session {
            emit: vec![Output::PTXAssembly],
            achitectures: vec![],

            opt_level: OptLevel::None,
            debug_info: false,

            output: Some(PathBuf::from("/kernel/target/debug/deps/libkernel.ptx")),

            include_rlibs: vec![
                PathBuf::from("/tmp/rustc.Ew934MzC8cj0/liblib-f0faab0dbaa9f7ef.rlib"),
                PathBuf::from("/tmp/rustc.Ew934MzC8cj0/libother-6b4931ba2f43f84b.rlib"),
            ],

            include_bitcode_modules: vec![PathBuf::from("/kernel/target/debug/deps/kernel.0.o")],
        };

        assert_eq!(
            CommandLineRequest::from(matches.expect("Unable to parse CLI arguments")),
            CommandLineRequest::Link(expected_session)
        );
    }

    #[test]
    fn it_should_parse_optimization() {
        let matches = get_app().get_matches_from_safe(vec![
            "legacy-ptx-linker",
            "-o",
            "/kernel/target/debug/deps/libkernel.ptx",
            "-O1",
        ]);

        let expected_session = Session {
            emit: vec![Output::PTXAssembly],
            achitectures: vec![],

            opt_level: OptLevel::Default,
            debug_info: false,

            output: Some(PathBuf::from("/kernel/target/debug/deps/libkernel.ptx")),

            include_rlibs: vec![],
            include_bitcode_modules: vec![],
        };

        assert_eq!(
            CommandLineRequest::from(matches.expect("Unable to parse CLI arguments")),
            CommandLineRequest::Link(expected_session)
        );
    }

    #[test]
    fn it_should_not_print_unknown_target_json() {
        let matches = get_app().get_matches_from_safe(vec![
            "legacy-ptx-linker",
            "print",
            "another-target-triple",
        ]);

        assert!(matches.is_err());
    }

    #[test]
    fn it_should_print_64bit_target_json() {
        let matches = get_app().get_matches_from_safe(vec![
            "legacy-ptx-linker",
            "print",
            "nvptx64-nvidia-cuda",
        ]);

        assert_eq!(
            CommandLineRequest::from(matches.expect("Unable to parse CLI arguments")),
            CommandLineRequest::Print64BitTargetJson
        );
    }

    #[test]
    fn it_should_print_32bit_target_json() {
        let matches = get_app().get_matches_from_safe(vec![
            "legacy-ptx-linker",
            "print",
            "nvptx-nvidia-cuda",
        ]);

        assert_eq!(
            CommandLineRequest::from(matches.expect("Unable to parse CLI arguments")),
            CommandLineRequest::Print32BitTargetJson
        );
    }

}
