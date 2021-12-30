use std::env;

use randompicklib::random_pick;

fn main() -> std::io::Result<()> {
    let target_dir = env::current_dir()?;
    let picked_file = random_pick(&target_dir);
    println!("{:?}", picked_file);
    Ok(())
}
