use std::env;

use randompicklib::random_pick;

fn main() -> std::io::Result<()> {
    let target_dir = env::current_dir()?;
    let picked_file = random_pick(&target_dir);
    match picked_file {
        Some(path) => println!("{:?}", path),
        None => panic!("no files"),
    }
    Ok(())
}
