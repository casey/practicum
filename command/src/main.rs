extern crate glob as glob_lib;

macro_rules! die {
  ($fmt:expr) => {print!(concat!($fmt, "\n")); std::process::exit(1)};
  ($fmt:expr, $($arg:tt)*) => {print!(concat!($fmt, "\n"), $($arg)*); std::process::exit(1)};
}

fn glob(pattern: &str) -> Vec<String> {
  match glob_lib::glob(pattern) {
    Ok(globs) => globs.filter_map(Result::ok).filter_map(|p| p.as_path().to_str()).map(|s| s.to_string()).collect(),
    Err(_) => return vec!{},
  }
}

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    die!("usage: {} list", args[0]);
  }

  let ref cmd = args[1];

  if cmd == "list" {
    list()
  }
}

fn list() {
  for entry in glob("modules/*") {
    println!("{}", entry);
  }
}
