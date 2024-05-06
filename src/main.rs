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

#[derive(Debug)]                                                                         //Should Line struct be in OutputT struct??? 
struct Line {
    line: String,
    matched: bool,
    expected: bool,
}

#[derive(Default)]
struct OutputT {
    lines: Vec<Line>,
}

fn string_to_out_t(s: String, o: &mut OutputT, exp: bool) {
    let v: Vec<_> = s.match_indices("\n").collect();

    let mut begin: usize = 0;
    let mut end: usize;    
    for (i, _) in v {
        end = i;
        let l = Line { line: s[begin..end].to_string(), matched: false, expected: exp };
        o.lines.push(l);
        begin = end + 1;
    }
}

#[allow(unused)]
fn cmp_order(o: &mut Line, e: &mut Line, no_space_format: bool) {

    //TODO trim all whitespaces then compare if no_space_format

    if o.line == e.line {
        o.matched = true;
        e.matched = true;
    }
}

#[allow(unused)]
fn cmp_disorder(s: &Line, e: &Vec<Line>, no_space_format: bool) {
    
}

fn print_result(prog_out_t: OutputT, exp_out_t: OutputT) {
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

    let mut it_prog_out = prog_out_t.lines.iter();
    let mut it_exp_out = exp_out_t.lines.iter();
    let mut i = 1;

    println!("{}\nLINES{}| {}ACTUAL OUTPUT{}|{}EXPECTED OUTPUT{}\n{}|{}|{}",
        "_".repeat(200), 
        " ".repeat(5), 
        " ".repeat(37), 
        " ".repeat(40), 
        " ".repeat(40), 
        " ".repeat(42),
        "_".repeat(10),
        "_".repeat(91),
        "_".repeat(97));

    loop {

        match (it_prog_out.next(), it_exp_out.next()) {
            (Some(prog_out_ln), Some(exp_out_ln)) => {
                let left_ofst: String = " ".repeat(10 - i.to_string().chars().count());
                let mid_ofst: String = " ".repeat(100 - i.to_string().chars().count() - left_ofst.chars().count() - prog_out_ln.line.chars().count());
                
                if !prog_out_ln.matched {
                    println!("{style_bold}{color_white}{}{style_reset}{}| {color_red}{}{color_reset}{}| {color_white}{}", i, left_ofst, prog_out_ln.line, mid_ofst, exp_out_ln.line);
                } else {
                    println!("{style_bold}{color_white}{}{style_reset}{}| {color_green}{}{color_reset}{}| {color_white}{}", i, left_ofst, prog_out_ln.line, mid_ofst, exp_out_ln.line);
                }
            },
            (Some(prog_out_ln), None) => {

            },
            (None, Some(exp_out_ln)) => {

            },
            (None, None) => break
        }
        i += 1;
    }
}

fn main() {
    let args: Args = Args::parse();
    //dbg!(&args);

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

    string_to_out_t(prog_out, &mut prog_out_t, false);         // is it correct here to not pass reference so the function sort of free content?
    string_to_out_t(exp_out, &mut exp_out_t, true);

    // dbg!(&prog_out_t.lines);
    // dbg!(&exp_out_t.lines);
    
    //loop that performs the test
    let mut it_prog_out = prog_out_t.lines.iter_mut();
    let mut it_exp_out = exp_out_t.lines.iter_mut();
    loop {
        match (&mut it_prog_out.next(), &mut it_exp_out.next()) {
            (Some(prog_out_ln), Some(exp_out_ln)) => {
                if args.no_line_order {
                    //TODO:
                    //cmp_disorder(&prog_out_t_l, &exp_out_t.lines, args.no_space_format);
                    //TODO remove following
                } else {
                    cmp_order(prog_out_ln, exp_out_ln, args.no_space_format);
                }
            },
            (Some(prog_out_ln), None) => prog_out_ln.expected = true,
            (None, Some(exp_out_ln)) => exp_out_ln.expected = false,
            (None, None) => break
        }
    }

    print_result(prog_out_t, exp_out_t);

    // dbg!(&prog_out_t.lines);
    // dbg!(&exp_out_t.lines);

}
