use clap::Parser;
use inline_colorization::*;

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

    #[clap(long = "line-order", default_value_t = false)]
    /// Will match even if lines of the output aren't found in the same order as expected (false by
    /// default)
    no_line_order: bool,

    #[clap(long = "space-format", default_value_t = false)]
    /// Will match even if whitespaces of the output aren't exactly as expected (false by default)
    no_space_format: bool,
}

#[derive(Default)]
struct OutputT {
    lines: Vec<String>,
}

fn string_to_out_t(s: String, o: &mut OutputT) {
    let v: Vec<_> = s.match_indices("\n").collect();

    let mut begin: usize = 0;
    let mut end: usize;
    for (i, _) in v {
        end = i;
        o.lines.push(s[begin..end].to_string());
        begin = end + 1;
    }
}

#[allow(unused)]
fn cmp_order(s: &String, e: &String, no_space_format: bool) -> u32 {

    //TODO trim all whitespaces then compare if no_space_format

    let mut i = 0;
    for c in s.chars() {
        if c != e.chars().nth(i).expect("Out of bound operation") {
            i += 1;
            return i.try_into().unwrap();
        }
        i += 1;
    }
    0
}

#[allow(unused)]
fn cmp_disorder(s: &String, e: &Vec<String>, no_space_format: bool) -> u32 {
    
    0
}

// TODO remove
fn debug_loop(mismatch_i: u32, prog_out_ln: &String) {
    //DEBUG
    if mismatch_i != 0 {
        println!("{color_red}Mismatch: {}", &prog_out_ln);
    } else {
        println!("{color_green}{}", &prog_out_ln)
    }
}


fn main() {
    let args: Args = Args::parse();
    dbg!(&args);

    // run sub-process and get output
    let prog_out: String = String::from_utf8(
        Command::new(&args.program)
            .args(&args.program_arguments)
            .output()
            .expect(&("failed to execute process ".to_owned() + &args.program))
            .stdout)
        .unwrap();

    // read expected output file
    let exp_out: String = fs::read_to_string(&args.expected)
        .expect(&("failed to open file ".to_owned() + &args.expected));

    //dbg!(&exp_out);
    //dbg!(&prog_out);
                                                                                // Maybe its better to have something like `let exp_out_t = string_to_out_t(exp_out)` ???
    let mut prog_out_t: OutputT = Default::default();
    let mut exp_out_t: OutputT = Default::default();

    string_to_out_t(prog_out, &mut prog_out_t);         // is it correct here to not pass reference so the function sort of free content?
    string_to_out_t(exp_out, &mut exp_out_t);

    dbg!(&prog_out_t.lines);
    dbg!(&exp_out_t.lines);

    #[allow(unused_assignments)]
    let mut mismatch_i: u32;

    //loop test
    let mut it_prog_out = prog_out_t.lines.iter();
    let mut it_exp_out = exp_out_t.lines.iter();

    loop {
        match (it_prog_out.next(), it_exp_out.next()) {

            (Some(prog_out_ln), Some(exp_out_ln)) => {
                if args.no_line_order {
                    //TODO:
                    //mismatch_i = cmp_disorder(&prog_out_t_l, &exp_out_t.lines, args.no_space_format);
                    //TODO remove following
                    mismatch_i = 0;
                } else {
                    mismatch_i = cmp_order(&prog_out_ln, &exp_out_ln, args.no_space_format);
                }

                debug_loop(mismatch_i, &prog_out_ln);
            },

            (Some(prog_out_ln), None) => {

            },

            (None, Some(prog_exp_ln)) => {

            },

            (None, None) => break
        }
        
        
    }

    //TODO:
    // print output/expected side by side with line number. Missing lines should be red in output
    //
    // if order:
    //      enumerate output/expected with line numbers
    //
    // if not_order:
    //      enumerate missing lines
    //
    //NOTE: 
    // whitespace option should differentiate between only whitespace difference or text difference
    //
    // if whitespace_diff_only:
    //      red padding for too many spaces in output/pointing arrow for where spaces were expected
}
