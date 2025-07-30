use clap::Parser;
mod cli_args;
use cli_args::Args;
fn main() {
    Args::init();

    println!("Done.  Completed in sec");
}
