#![feature(phase)]
extern crate serialize;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;
extern crate color;

use docopt::FlagParser;

docopt!(Args, "
Usage: hexlet-lesson [options] login [KEY]
       hexlet-lesson [options] init [LESSON LANGUAGE]
       hexlet-lesson [options] submit [PATH]

Options:
  --host TEXT
  -v, --verbose
  -h, --help         Show this message and exit.

Commands:
  login
  init
  submit
")

fn main() {
    let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());
    println!("{}", args);

    //    let verbose: bool = args.flag_verbose;

    // lesson::init("test", "ru");

    //    logger::info("bla", verbose);
    //    logger::debug("bla", verbose);
}
