#![feature(phase)]
extern crate serialize;
#[phase(plugin)] extern crate docopt_macros;
#[phase(plugin, link)] extern crate log;
extern crate docopt;
extern crate color;
extern crate http;
extern crate url;

use std::os;
use docopt::FlagParser;

mod lesson;
mod logger;


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

    if args.flag_verbose {
      os::setenv("RUST_LOG", "info");
    }

    if args.cmd_init {
        let lesson_name = args.arg_LESSON.as_slice();
        let lesson_language = args.arg_LANGUAGE.as_slice();

        info!("Run lesson init with args: {}, {}", lesson_name, lesson_language)
        lesson::init(lesson_name, lesson_language);
    }

    if args.cmd_login {
        let hexlet_api_key = args.arg_KEY.as_slice();

        info!("Run login with api_key {}", hexlet_api_key)
        lesson::login(hexlet_api_key);
    }

    if args.cmd_submit {
        let lesson_path = args.arg_PATH.as_slice();

        info!("Run lesson {} submit with", lesson_path)
        lesson::submit(lesson_path);
    }
}
