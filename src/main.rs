use diff::run;
use diff::Paths;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Paths::new(&args)?;
    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        return Err(err);
    };
    Ok(())
}
