#![crate_id(name="yes", vers="1.0.0", author="Seldaek")]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Jordi Boggiano <j.boggiano@seld.be>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/* last synced with: yes (GNU coreutils) 8.13 */

#![feature(macro_rules)]

extern crate getopts;
extern crate libc;

use std::os;
use std::io::{print, println};

#[path = "../common/util.rs"]
mod util;

static NAME: &'static str = "yes";

#[allow(dead_code)]
fn main() { os::set_exit_status(uumain(os::args())); }

pub fn uumain(args: Vec<String>) -> int {
    let program = args.get(0).clone();
    let opts = [
        getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("V", "version", "output version information and exit"),
    ];
    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => {
            crash!(1, "invalid options\n{}", f.to_err_msg())
        }
    };
    if matches.opt_present("help") {
        println!("yes 1.0.0");
        println!("");
        println!("Usage:");
        println!("  {0:s} [STRING]... [OPTION]...", program);
        println!("");
        print(getopts::usage("Repeatedly output a line with all specified STRING(s), or 'y'.", opts).as_slice());
        return 0;
    }
    if matches.opt_present("version") {
        println!("yes 1.0.0");
        return 0;
    }
    let mut string = "y".to_string();
    if !matches.free.is_empty() {
        string = matches.free.connect(" ");
    }

    exec(string.as_slice());

    0
}

pub fn exec(string: &str) {
    loop {
        println(string);
    }
}
