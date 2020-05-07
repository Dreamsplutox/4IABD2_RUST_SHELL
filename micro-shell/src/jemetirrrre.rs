// use std::io::{self, Write};
// use std::process::{Command};

use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {
  let ls = Command::new("ls")
    .arg("-alh")
    .stdout(Stdio::piped())
    .spawn()
    .expect("I was pancaked while trying to launch ls.");

  let ls_stdout = Stdio::from(ls.stdout.expect("Something wrong with ls stdin"));
    // Stdio::piped() -> type pour representer une entrée/sortie standard qui sera un tube.
  let process = Command::new("wc")
    .arg("--lines")
    .stdin(ls_stdout)
    .spawn()
    .expect("Whopsie! wc failled to launch");
}