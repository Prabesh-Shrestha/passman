use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

pub fn handle_cmd () {

    let args = Args::parse();
}
