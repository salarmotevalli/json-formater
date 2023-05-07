// use std::fs::metadata;
// use std::os::unix::io::AsRawFd;
// use std::io;
// fn main() -> io::Result<()> {
//     let stdin = io::stdin();
//     let fd = stdin.as_raw_fd();
//     let meta = metadata("/dev/fd/".to_owned() + &fd.to_string())?;
//     println!("File type: {:?}", meta.file_type());
//     Ok(())
// }

// use crate::flags;

// mod flags;

use std::process;

mod flag_manager;

fn main() {
    // Parse flags
    parse_flags();
    
    // Validate entry  
    if flag_manager::env::passed_flags_count() == 0 {
        flag_manager::display::err("Not enough arguments", Some("You have to pass at least one argument"));
        flag_manager::display::hello();
        process::exit(1);
    }

    
}

fn parse_flags() {
    let _ = flag_manager::parser::new("i", None, Some("define input file"));
}