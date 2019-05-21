use std::env;

pub fn print_current_dir() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("Current directory is {}", path.display());
    Ok(())
}
