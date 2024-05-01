use clap::Parser;

use std::process::Command;
use std::fs;

#[derive(Parser, Debug)]
#[clap(author = "Thomas Laflamme", version = "0.1", about = "Utility to compare your program's output to the expected output")]
struct Args {
    /// The program to test.
    program: String,

    /// The program's arguments.
    program_arguments: Vec<String>,

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

#[derive(Default)]
struct OutputType {
    lines: Vec<String>,
}

fn string_to_output_type(s: String, o: &mut OutputType) {
    let v: Vec<_> = s.match_indices("\n").collect();
    let mut begin: usize = 0;
    let mut end: usize;
    for j in v {
        end = j.0;
        o.lines.push(s[begin..end].to_string());
        begin = end + 1;
    }
    //dbg!(o.lines);
}

fn main() {
    let args: Args = Args::parse();
    //dbg!(args);

    // run sub-process and get output
    let program_output_content: String = String::from_utf8(
        Command::new(&args.program)
            .args(&args.program_arguments)
            .output()
            .expect(&("failed to execute process ".to_owned() + &args.program))
            .stdout)
        .unwrap();

    // read expected output file
    let expected_output_content: String = fs::read_to_string(&args.expected)
        .expect(&("failed to open file ".to_owned() + &args.expected));

    //dbg!(&expected_output_content);
    //dbg!(&program_output_content);

    // Maybe its better to have something like `let expected_output =
    // string_to_output_type(expected_output_content)`
    let mut program_output: OutputType = Default::default();
    let mut expected_output: OutputType = Default::default();
    string_to_output_type(program_output_content, &mut program_output);
    string_to_output_type(expected_output_content, &mut expected_output);

    dbg!(program_output.lines);
    dbg!(expected_output.lines);
}
