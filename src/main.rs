#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate rustbox;
extern crate mark;

use std::default::Default;
use std::path::PathBuf;

use rustbox::RustBox;
use docopt::Docopt;
use mark::Editor;


const USAGE: &'static str = "
Mark.

Usage:
  mark [<filename>]
";

#[derive(Debug, Deserialize)]
struct Args {
  arg_filename: Option<String>
}

fn main() {
  let args: Args = Docopt::new(USAGE)
      .and_then(|docopt| docopt.deserialize())
      .unwrap_or_else(|e| e.exit());

  let rustbox = match RustBox::init(Default::default()) {
    Result::Ok(v) => v,
    Result::Err(e) => panic!("{}", e),
  };

  let mut editor = Editor::new(PathBuf::from(args.arg_filename.unwrap()), rustbox);

  editor.start();
}
