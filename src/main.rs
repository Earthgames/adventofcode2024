use clap::Parser;

mod match_day;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    /// The day
    #[clap(default_value_t = 1)]
    #[arg(short, long)]
    pub day: u8,

    /// part two
    #[clap(default_value_t = false)]
    #[clap(short, long)]
    pub part_two: bool,

    /// use test input
    #[clap(default_value_t = false)]
    #[clap(short, long)]
    pub test: bool,
}

fn main() {
    let cli = Cli::parse();
    let (result, time) = match_day::match_day(cli.day, cli.part_two, cli.test);
    let part = if cli.part_two { "two" } else { "one" };
    match result {
        Ok(output) => println!(
            "day {0} part {part} in {1}ms\n\n{output}",
            cli.day,
            time.as_millis(),
        ),
        Err(err) => println!(
            "there was an error: \"{err}\"\nin day {0} part {part}",
            cli.day,
        ),
    }
}
