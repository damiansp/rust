use resplit::Cli;
use clap::Parser;


fn main() {
    let cli = Cli::parse();
    let buffer = resplit::read_stdin();
    let res = resplit::split(buffer, &cli);
    println!("{res}");
}
