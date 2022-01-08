use std::env;

use randompicklib::random_pick;

fn main() -> std::io::Result<()> {
    let target_dir = env::current_dir()?;
    let picked_file = random_pick(&target_dir);
    match picked_file {
        Some(path) => match path.to_str() {
            Some(path_str) => println!("{}", path_str),
            None => panic!("invalid unicode file name: {:?}", path),
        },
        None => panic!("no files"),
    }
    Ok(())
}
