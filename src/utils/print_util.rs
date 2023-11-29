use ansi_term::Colour;
use ansi_term::Colour::{Blue, Green, Red, Yellow};

pub fn print_logo() {
    let art = r"
  ____    _    _ __     _____      ____ _     ___ 
 / ___|  / \  | |\ \   / / _ \    / ___| |   |_ _|
 \___ \ / _ \ | | \ \ / / | | |  | |   | |    | | 
  ___) / ___ \| |__\ V /| |_| |  | |___| |___ | | 
 |____/_/   \_\_____\_/  \___/    \____|_____|___|
                                                 
";
    let lines = art.lines();
    for line in lines {
        let (part_blue, part_green) = line.split_at(line.len() / 2);
        println!("{}{}", Blue.paint(part_blue), Green.paint(part_green));
    }
    println!(); // a new line
}

pub fn warning<S: AsRef<str>>(msg: S) {
    println!("{}", Yellow.paint(msg.as_ref()));
}
pub fn error<S: AsRef<str>>(msg: S) {
    println!("{}", Red.paint(msg.as_ref()));
}
pub fn success<S: AsRef<str>>(msg: S) {
    println!("{}", Blue.paint(msg.as_ref()));
}
pub fn orange<S: AsRef<str>>(msg: S) {
    println!("{}", Colour::RGB(255, 165, 0).paint(msg.as_ref()));
}
pub fn green<S: AsRef<str>>(msg: S) {
    println!("{}", Green.paint(msg.as_ref()));
}
