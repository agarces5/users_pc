use clap::Parser;
use cli::Args;

mod cli;
mod users;

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    args.execute()?;

    Ok(())
}
