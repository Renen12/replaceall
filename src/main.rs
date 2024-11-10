use std::{
    env::{self, args},
    fs,
    process::exit,
};
fn main() {
    let mut args: Vec<String> = args().collect();
    args.remove(0);
    if args.len() < 2 {
        eprintln!("replaceall - [DIRECTORY TO REPLACE ALL IN] [FILE TO SOURCE FROM]");
        exit(1);
    }
    let dir = &args[0];
    for file in fs::read_dir(dir).unwrap() {
        let file = file.unwrap();
        let contents = fs::read(&args[1]).unwrap();
        env::set_current_dir(dir).unwrap();
        fs::write(
            file.path().into_os_string().into_string().unwrap(),
            contents,
        )
        .unwrap();
    }
}
