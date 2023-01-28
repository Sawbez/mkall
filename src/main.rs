use clap::Parser;
use exitcode::{ExitCode, CANTCREAT, IOERR, OK, TEMPFAIL};

use std::fs::create_dir_all;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::exit;

struct ErrMsg<'a>(&'a str, ExitCode);

// A program to mkdir and cd at the same time
#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Mkcd {
    /// List of paths to create
    paths: Vec<PathBuf>,
}

fn get_err_msg(err: Error) -> ErrMsg<'static> {
    match err.kind() {
        ErrorKind::PermissionDenied => {
            ErrMsg("Permission to create the folder was denied.", CANTCREAT)
        }
        ErrorKind::AlreadyExists => {
            ErrMsg("One of the specified folders already exists.", CANTCREAT)
        }
        ErrorKind::InvalidData => ErrMsg("The filename is invalid.", CANTCREAT),
        ErrorKind::Unsupported => ErrMsg("File creation is unsupported.", CANTCREAT),
        ErrorKind::TimedOut => ErrMsg("The operation timed out.", TEMPFAIL),
        ErrorKind::Interrupted => ErrMsg("The file creation timed out.", TEMPFAIL),
        ErrorKind::OutOfMemory => ErrMsg(
            "There is not enough memory to complete the creation.",
            TEMPFAIL,
        ),
        _ => ErrMsg("An unknown error occured.", IOERR),
    }
}

fn main() {
    let paths = Mkcd::parse().paths;

    for path in paths {
        let err_msg = match create_dir_all(path) {
            Err(err) => Some(get_err_msg(err)),
            Ok(_) => None,
        };

        if let Some(msg) = err_msg {
            let ErrMsg(out_msg, exit_code) = msg;
            eprintln!("{out_msg}");
            exit(exit_code);
        };
    }

    exit(OK);
}
