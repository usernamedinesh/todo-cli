use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,
    key: String,
}

fn main() {
    let args = Cli::parse();
    println!("command: {} key : {}", args.command, args.key);
}
