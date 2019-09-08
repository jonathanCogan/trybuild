extern crate term;
extern crate difference;
use difference::{Difference, Changeset};
use std::io::Write;

pub fn print_diff(expected: &str, actual: &str) {
    println!("\n\nEXPECTED: ");
    println!("{}", "┈".repeat(60));
    let Changeset { diffs, .. } = Changeset::new(expected, actual, "\n");
    let mut t = term::stdout().unwrap();
    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref x) => {
                t.fg(term::color::WHITE).unwrap();
                let s = x.replace('\n', "\n ");
                writeln!(t, " {}", s.as_str());
            }
            Difference::Add(ref x) => {
                t.fg(term::color::GREEN).unwrap();
                let s = x.replace('\n', "\n+");
                writeln!(t, "+{}", s.as_str());
            }
            Difference::Rem(ref x) => {
                t.fg(term::color::RED).unwrap();
                let s = x.replace('\n', "\n-");
                writeln!(t, "-{}", s.as_str());
            }
        }
    }
    t.reset().unwrap();
    t.flush().unwrap();
    println!("{}", "┈".repeat(60));
}