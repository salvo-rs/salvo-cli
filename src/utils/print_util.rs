use ansi_term::Colour::{Blue, Green, Red, Yellow};

pub fn print_logo() {
    let art = r"
    _______________________    ________     ______________________
    __  ___/__    |__  /__ |  / /_  __ \    __  ____/__  /____  _/
    _____ \__  /| |_  / __ | / /_  / / /    _  /    __  /  __  /  
    ____/ /_  ___ |  /____ |/ / / /_/ /     / /___  _  /____/ /   
    /____/ /_/  |_/_____/____/  \____/      \____/  /_____/___/   
";
    let lines = art.lines();
    for line in lines {
        let (part_blue, part_green) = line.split_at(line.len() / 2);
        println!("{}{}", Blue.paint(part_blue), Green.paint(part_green));
    }
}

pub fn warning(msg: String) {
    println!("{}", Yellow.paint(msg));
}
pub fn error(msg: String) {
    println!("{}", Red.paint(msg));
}
