use clap::Parser;

use std::process::Command;

#[derive(Parser, Debug)]
#[clap(author = "Thomas Laflamme", version = "0.1", about = "Utility to compare your program's output to the expected output")]
struct Args {
    /// The program to test.
    prog: String,

    /// The program's arguments.
    prog_args: Vec<String>,

    #[clap(short = 'e', long = "expected")]
    /// The expected output file.
    expected: String,

    #[clap(long = "line-order", default_value_t = true)]
    /// Should every line be in the exact same order as the expected output. True by default.
    line_order: bool,

    #[clap(long = "space-format", default_value_t = true)]
    /// Should a whitespace difference count. True by default.
    space_format: bool,
}

fn main() {
    let args = Args::parse();
    //dbg!(args);

    let prog_output = Command::new(args.prog)
        .args(args.prog_args)
        .output()
        .expect("failed to execute process");

    dbg!(prog_output.stdout);
}
