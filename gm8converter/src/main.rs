#![feature(try_trait_v2)]

use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::str::FromStr;

pub mod utils;
pub mod cpp_backend;

use gm8decompiler::deobfuscate;
use gm8exe::GameAssets;

use crate::{
    cpp_backend::*,
    utils::{ParsingError, ToParsingError}
};

fn decompile(
    in_path: &Path,
    verbose: bool,
    strict: bool,
    multithread: bool,
    deobf_mode: deobfuscate::Mode,
    fix_events: bool,
) -> Result<(), ParsingError> {
    // slurp in file contents
    let file = fs::read(&in_path).convert()?;

    // parse (entire) gamedata
    let logger = if verbose { Some(|msg: &str| println!("{}", msg)) } else { None };
    let mut assets = gm8exe::reader::from_exe(file, logger, strict, multithread)
        .convert()?;

    println!("Successfully parsed game!");

    //Do we want to deobfuscate, yes or no?
    let deobfuscate = match deobf_mode {
        deobfuscate::Mode::On => true,
        deobfuscate::Mode::Off => false,
        deobfuscate::Mode::Auto => {
            assets.backgrounds.iter().flatten().any(|s| s.name.0.is_empty())
                || assets.fonts.iter().flatten().any(|s| s.name.0.is_empty())
                || assets.objects.iter().flatten().any(|s| s.name.0.is_empty())
                || assets.paths.iter().flatten().any(|s| s.name.0.is_empty())
                || assets.rooms.iter().flatten().any(|s| s.name.0.is_empty())
                || assets.sounds.iter().flatten().any(|s| s.name.0.is_empty())
                || assets.sprites.iter().flatten().any(|s| s.name.0.is_empty())
                || assets.timelines.iter().flatten().any(|s| s.name.0.is_empty())
        },
    };
    if deobf_mode == deobfuscate::Mode::Auto && deobfuscate {
        println!("Note: GMK looks obfuscated, so de-obfuscation has been enabled by default");
        println!(" -- you can turn this off with '-d off'");
    }

    fn fix_event(ev: &mut gm8exe::asset::CodeAction) {
        // So far the only broken event type I know of is custom Execute Code actions.
        // We can fix these by changing the act id and lib id to be a default Execute Code action instead.
        if ev.action_kind == 7 && ev.execution_type == 2 {
            // 7 = code block param, 2 = code execution
            ev.id = 603;
            ev.lib_id = 1;
        }
    }

    if fix_events {
        assets
            .objects
            .iter_mut()
            .flatten()
            .flat_map(|x| x.events.iter_mut().flatten())
            .flat_map(|(_, x)| x.iter_mut())
            .for_each(|ev| fix_event(ev));

        assets
            .timelines
            .iter_mut()
            .flatten()
            .flat_map(|x| x.moments.iter_mut().flat_map(|(_, x)| x.iter_mut()))
            .for_each(|ev| fix_event(ev));
    }

    if deobfuscate {
        deobfuscate::process(&mut assets);
    }
    
    assets.to_dir(&String::from_str("output/").unwrap())
}

// Know to "press any key" but only if double-clicked in WinExplorer or whatever.
#[cfg(windows)]
fn is_cmd(argv_0: &str) -> bool {
    let is_argv0_absolute = Path::new(argv_0).is_absolute();
    let is_msys2 = env::var("MSYSTEM").is_ok();

    is_argv0_absolute && !is_msys2
}
#[cfg(windows)]
fn pause(tip: bool) {
    extern "C" {
        fn _getch() -> std::os::raw::c_int;
    }
    if tip {
        println!("\nTip: To decompile a game, click and drag it on top of the executable.");
    }
    println!("<< Press Any Key >>");
    let _ = unsafe { _getch() };
}
#[cfg(not(windows))]
fn is_cmd(_argv_0: &str) -> bool {
    false
}
#[cfg(not(windows))]
fn pause(_tip: bool) {}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(!args.is_empty());
    let process_path = args[0].as_str();
    let should_pause = is_cmd(process_path);

    // set up getopts to parse our command line args
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print this help message")
        .optflag("l", "lazy", "disable various data integrity checks")
        .optflag("v", "verbose", "enable verbose logging for decompilation")
        .optopt("d", "deobfuscate", "set deobfuscation mode auto/on/off (default=auto)", "")
        .optflag("p", "preserve", "preserve broken events (instead of trying to fix them)")
        .optflag("s", "singlethread", "decompile gamedata synchronously (lower RAM usage)")
        .optopt("o", "output", "specify output filename", "FILE");

    // parse command line arguments
    let matches = match opts.parse(&args[1..]) {
        Ok(matches) => matches,
        Err(err) => {
            use getopts::Fail::*;
            match err {
                ArgumentMissing(arg) => eprintln!("Missing argument: {}", arg),
                UnrecognizedOption(opt) => eprintln!("Unrecognized option: {}", opt),
                OptionMissing(opt) => eprintln!("Missing option: {}", opt),
                OptionDuplicated(opt) => eprintln!("Duplicated option: {}", opt),
                UnexpectedArgument(arg) => eprintln!("Unexpected argument: {}", arg),
            }
            if should_pause {
                pause(true);
            }
            process::exit(1);
        },
    };

    // print help message if requested OR no input files
    if matches.opt_present("h") || matches.free.is_empty() {
        // If the getopts2 usage generator didn't suck this much,
        // I wouldn't have to resort to this.
        // TODO: Get a better argument parser in general.
        println!(
            "Usage: {} FILENAME [options]

Options:
    -h, --help                print this help message
    -l, --lazy                disable various data integrity checks
    -v, --verbose             enable verbose logging for decompilation
    -d, --deobfuscate <mode>  set deobfuscation mode auto/on/off (defaults to auto)
    -p, --preserve            preserve broken events (instead of trying to fix them)
    -s, --singlethread        decompile gamedata synchronously (lower RAM usage)
    -o, --output <file>       specify output filename",
            process_path
        );
        if should_pause {
            pause(true);
        }
        process::exit(0); // once the user RTFM they can run it again
    }

    // print error message if multiple inputs were provided
    if matches.free.len() > 1 {
        eprintln!(
            concat!("Unexpected input: {}\n", "Tip: Only one input gamefile is expected at a time!",),
            matches.free[1]
        );
        if should_pause {
            pause(true);
        }
        process::exit(1);
    }

    // extract flags & input path
    let input = &matches.free[0];
    let lazy = matches.opt_present("l");
    let singlethread = matches.opt_present("s");
    let verbose = matches.opt_present("v");
    let deobfuscate = match matches.opt_str("d").as_deref() {
        Some("on") => deobfuscate::Mode::On,
        Some("off") => deobfuscate::Mode::Off,
        Some("auto") | None => deobfuscate::Mode::Auto,
        Some(x) => {
            eprintln!("Invalid deobfuscator setting: {} (valid settings are on/off/auto)", x);
            process::exit(1);
        },
    };
    let out_path = matches.opt_str("o");
    let preserve = matches.opt_present("p");
    // no_pause extracted before help

    // print flags for confirmation
    println!("Input file: {}", input);
    if lazy {
        println!("Lazy mode ON: data integrity checking disabled");
    }
    if verbose {
        println!("Verbose logging ON: verbose console output enabled");
    }
    match deobfuscate {
        deobfuscate::Mode::On => println!("Deobfuscation ON: will standardise GML code"),
        deobfuscate::Mode::Off => println!("Deobfuscation OFF: will ignore obfuscation"),
        _ => (),
    }
    if singlethread {
        println!("Single-threaded mode ON: process will not start new threads (slow)");
    }
    if let Some(path) = &out_path {
        println!("Specified output path: {}", path);
    }
    if preserve {
        println!("Preserve mode ON: broken events will be preserved and will not be fixed");
    }

    // resolve input path
    let input_path = Path::new(input);
    if !input_path.is_file() {
        eprintln!("Input file '{}' does not exist.", input);
        process::exit(1);
    }

    match decompile(input_path, !lazy, !singlethread, verbose, deobfuscate, !preserve) {
    Err(e) =>
        eprintln!("Error while decompiling the executable: {}", e),
    Ok(_) => {}
    }
    /*
    match Project::from_dir(&args[1]) {
        Err(ParsingError::DecodingError(e)) => {
            eprintln!("Error while decoding: {e}");
            process::exit(1);
        } 
        Err(ParsingError::FileError(e)) => {
            eprintln!("Error while reading: {e}");
            process::exit(1);
        } 
        Err(ParsingError::ImageDecodingError(e)) => {
            eprintln!("Error while decoding an image: {e}");
            process::exit(1);
        }
        Ok(prj) => {
            match prj.to_dir(&args[2]) {
            Ok(_) => {
                dbg!(&prj);
            }
            Err(ParsingError::DecodingError(e)) => {
                eprintln!("Error while decoding: {e}");
                process::exit(1);
            } 
            Err(ParsingError::FileError(e)) => {
                eprintln!("Error while reading: {e}");
                process::exit(1);
            }
            Err(ParsingError::ImageDecodingError(e)) => {
                eprintln!("Error while decoding an image: {e}");
                process::exit(1);
            }
            }
        } 
    }
    */
}
