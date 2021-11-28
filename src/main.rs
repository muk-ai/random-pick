use std::env;

use randompicklib::get_files;

fn main() -> std::io::Result<()> {
    let target_dir = env::current_dir()?;
    let list = get_files(&target_dir)?;
    for path in list {
        println!("{:?}", path);
    }
    Ok(())
}
