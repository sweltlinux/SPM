use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: install,

    #[arg(short, long)]
    name: update,

    #[arg(short, long)]
    name: upgrade,

    #[arg(short, long)]
    name: search,

    #[arg(short, long)]
    name: remove

}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
