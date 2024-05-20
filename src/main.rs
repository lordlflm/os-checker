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

    #[clap(long = "no-line-order", default_value_t = false)]
    /// Will match even if lines of the output aren't found in the same order as expected (false by
    /// default)
    no_line_order: bool,

    #[clap(long = "no-space-format", default_value_t = false)]
    /// Will match even if whitespaces of the output aren't exactly as expected (false by default)
    no_space_format: bool,
}

#[derive(Debug, Clone)]
struct Line {
    line: String,
    matched: bool,
}

struct OutputT {
    lines: Vec<Line>,
}

impl OutputT {
    fn new(out_string: String) -> Self {
        let mut lines = Vec::<Line>::new();

        let v: Vec<_> = out_string.match_indices("\n").collect();
        let mut begin: usize = 0;
        let mut end: usize;

        for (i, _) in v {
            end = i;
            let l = Line { line: out_string[begin..end].to_string(), matched: false };
            lines.push(l);
            begin = end + 1;
        }

        Self {
            lines: lines
        }
    }
}

fn print_result(prog_out_t: OutputT, exp_out_t: OutputT, no_line_order: bool) {
    //TODO
    // 1. make output more conveniant for different screen sizes
    
    let mut summary_string_expected_vec = Vec::<String>::new();
    let mut summary_string_unexpected_vec = Vec::<String>::new();
    let mut it_prog_out = prog_out_t.lines.iter();
    let mut it_exp_out = exp_out_t.lines.iter();
    let mut mismatches = 0;
    let mut i = 1;
    
    //print actual output and expected output side-by-side
    println!("COMPARISON\n{}\nLINES{}| {}ACTUAL OUTPUT{}|{}EXPECTED OUTPUT{}\n{}|{}|{}",
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
                    println!("{style_bold}{color_white}{}{style_reset}{}| {color_red}{}{color_reset}{}| {color_white}{}{color_reset}",
                        i, 
                        left_ofst, 
                        prog_out_ln.line, 
                        mid_ofst, 
                        exp_out_ln.line);
                    
                    if !no_line_order {
                        let ofst = " ".repeat(4 + i.to_string().chars().count());
                        let summary_string = format!("At line {} got: {color_red}\"{}\"{color_reset}\n{}expected: {color_red}\"{}\"{color_reset}\n", i, prog_out_ln.line, ofst, exp_out_ln.line);
                        summary_string_expected_vec.push(summary_string);
                    } else {
                        let summary_string = format!("Did not expect {color_red}\"{}\"{color_reset} (which was found at line {})", prog_out_ln.line, i);
                        summary_string_unexpected_vec.push(summary_string);
                    }

                    mismatches += 1;
                } else {
                    println!("{style_bold}{color_white}{}{style_reset}{}| {color_green}{}{color_reset}{}| {color_white}{}{color_reset}", 
                        i, 
                        left_ofst, 
                        prog_out_ln.line, 
                        mid_ofst, 
                        exp_out_ln.line);
                }

                if !exp_out_ln.matched && no_line_order {
                    let summary_string = format!("Expected {color_red}\"{}\"{color_reset} (found at line {} of expected output)", exp_out_ln.line, i);
                    summary_string_expected_vec.push(summary_string);
                }
            },
            (Some(prog_out_ln), None) => {
                let left_ofst: String = " ".repeat(10 - i.to_string().chars().count());
                let mid_ofst: String = " ".repeat(100 - i.to_string().chars().count() - left_ofst.chars().count() - prog_out_ln.line.chars().count());

                if !prog_out_ln.matched {
                    println!("{style_bold}{color_white}{}{style_reset}{}| {color_red}{}{color_reset}{}|", 
                        i, 
                        left_ofst, 
                        prog_out_ln.line, 
                        mid_ofst);

                    if !no_line_order {
                        let ofst = " ".repeat(4 + i.to_string().chars().count());
                        let summary_string = format!("At line {} got: {color_red}\"{}\"{color_reset}\n{}expected: {color_red}\"{}\"{color_reset}\n", i, prog_out_ln.line, ofst, "");
                        summary_string_expected_vec.push(summary_string);
                    } else {
                        let summary_string = format!("Did not expect {color_red}\"{}\"{color_reset} (which was found at line {})", prog_out_ln.line, i);
                        summary_string_unexpected_vec.push(summary_string);
                    }


                    mismatches += 1;
                } else {
                    println!("{style_bold}{color_white}{}{style_reset}{}| {color_green}{}{color_reset}{}|", 
                        i, 
                        left_ofst, 
                        prog_out_ln.line, 
                        mid_ofst);
                }
            },
            (None, Some(exp_out_ln)) => {
                let left_ofst: String = " ".repeat(10 - i.to_string().chars().count());
                let mid_ofst: String = " ".repeat(100 - i.to_string().chars().count() - left_ofst.chars().count());
                
                if !exp_out_ln.matched {
                    println!("{style_bold}{color_white}{}{style_reset}{}| {}| {color_red}{}{color_reset}", 
                        i, 
                        left_ofst, 
                        mid_ofst, 
                        exp_out_ln.line);

                    if !no_line_order {
                        let ofst = " ".repeat(4 + i.to_string().chars().count());
                        let summary_string = format!("At line {} got: {color_red}\"{}\"{color_reset}\n{}expected: {color_red}\"{}\"{color_reset}\n", i, "", ofst, exp_out_ln.line);
                        summary_string_expected_vec.push(summary_string);
                    } else {
                        let summary_string = format!("Expected {color_red}\"{}\"{color_reset} (found at line {} of expected output)", exp_out_ln.line, i);
                        summary_string_expected_vec.push(summary_string);
                    }

                    mismatches += 1;
                } else {
                    println!("{style_bold}{color_white}{}{style_reset}{}| {}| {color_green}{}{color_reset}", 
                        i, 
                        left_ofst, 
                        mid_ofst, 
                        exp_out_ln.line);
                }
            },
            (None, None) => break
        }
        i += 1;
    }
    println!("{}|{}|{}", "_".repeat(10), "_".repeat(91), "_".repeat(97));

    // print summary
    print!("\nSUMMARY: ");
    if mismatches == 0 {
        println!("{color_green}Found 0 mismatch !\n{color_reset}");
    } else {
        if mismatches == 1 {
            println!("{color_red}Found 1 mismatch\n{color_reset}");
        } else {
            println!("{color_red}Found {} mismatches\n{color_reset}", mismatches);
        }
    }

    for s in summary_string_expected_vec {
        println!("{}", s);
    }
    println!();
    for s in summary_string_unexpected_vec {
        println!("{}", s);
    }
}

fn main() {
    let args: Args = Args::parse();

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

    let mut prog_out_t: OutputT = OutputT::new(prog_out);
    let mut exp_out_t: OutputT = OutputT::new(exp_out);
    
    // perform the test
    if args.no_line_order {
        for prog_out_ln in &mut prog_out_t.lines {
            let mut prog_out_line_tmp = prog_out_ln.line.clone();

            for exp_out_ln in &mut exp_out_t.lines {
                let mut exp_out_line_tmp = exp_out_ln.line.clone();

                if args.no_space_format {
                    prog_out_line_tmp.retain(|c| !c.is_whitespace());
                    exp_out_line_tmp.retain(|c| !c.is_whitespace());
                }

                if prog_out_line_tmp == exp_out_line_tmp && !exp_out_ln.matched {
                    prog_out_ln.matched = true;
                    exp_out_ln.matched = true;
                    break;
                }
            }
        }
    } else {
        let mut it_prog_out = prog_out_t.lines.iter_mut();
        let mut it_exp_out = exp_out_t.lines.iter_mut();

        loop {
            match (&mut it_prog_out.next(), &mut it_exp_out.next()) {
                (Some(prog_out_ln), Some(exp_out_ln)) => {
                    let mut prog_out_line_tmp = prog_out_ln.line.clone();
                    let mut exp_out_line_tmp = exp_out_ln.line.clone();

                    if args.no_space_format {
                        prog_out_line_tmp.retain(|c| !c.is_whitespace());
                        exp_out_line_tmp.retain(|c| !c.is_whitespace());
                    }

                    if prog_out_line_tmp == exp_out_line_tmp {
                        prog_out_ln.matched = true;
                        exp_out_ln.matched = true;
                    }
                },
                (Some(_prog_out_ln), None) => continue,
                (None, Some(_exp_out_ln)) => continue,
                (None, None) => break
            }
        }
    }

    print_result(prog_out_t, exp_out_t, args.no_line_order);
}
