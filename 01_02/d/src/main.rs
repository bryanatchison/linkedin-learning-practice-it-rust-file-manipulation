use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let file_path: &str = "test_file";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{contents}");
    Ok(())
}
