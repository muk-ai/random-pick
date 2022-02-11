use std::env;

use randompicklib::random_pick;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().into_iter().skip(1).collect();
    let mut extensions: Vec<String> = vec![];
    for chunk in args.chunks(2) {
        if let [name, value] = chunk {
            if name == "-e" {
                extensions.push(value.clone())
            }
        }
    }
    println!("{extensions:?}");

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
