mod args;

use args::BoreArgs;
use clap::Parser;
fn main() {
    let args = BoreArgs::parse();
    println!("{:?}", args);
}