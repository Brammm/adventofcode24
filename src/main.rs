mod day1;
mod day2;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: String,
}

fn main() {
    let args = Args::parse();

    match args.day.as_str() {
        "day1" => day1::day1::run(),
        "day2" => day2::day2::run(),
        _ => eprintln!("Day {} not implemented", args.day),
    }
}
