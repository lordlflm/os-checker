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

#[derive(Debug, Clone)]                                                                         //Should Line struct be in OutputT struct??? 
struct Line {
    line: String,
    matched: bool,
}

struct OutputT {
    lines: Vec<Line>,
}

impl OutputT {
    fn new(out_string: String) -> Self {
        let v: Vec<_> = out_string.match_indices("\n").collect();
        let mut lines = Vec::<Line>::new();
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

fn string_to_out_t(s: String, o: &mut OutputT) {
    let v: Vec<_> = s.match_indices("\n").collect();

    let mut begin: usize = 0;
    let mut end: usize;    
    for (i, _) in v {
        end = i;
        let l = Line { line: s[begin..end].to_string(), matched: false };
        o.lines.push(l);
        begin = end + 1;
    }
}

fn print_result(prog_out_t: OutputT, exp_out_t: OutputT) {
    //TODO
    // 1. make output more conveniant for different screen sizes

    let mut it_prog_out = prog_out_t.lines.iter();
    let mut it_exp_out = exp_out_t.lines.iter();
    let mut mismatches = 0;
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
                    println!("{style_bold}{color_white}{}{style_reset}{}| {color_red}{}{color_reset}{}| {color_white}{}{color_reset}",
                        i, 
                        left_ofst, 
                        prog_out_ln.line, 
                        mid_ofst, 
                        exp_out_ln.line);
                    mismatches += 1;
                } else {
                    println!("{style_bold}{color_white}{}{style_reset}{}| {color_green}{}{color_reset}{}| {color_white}{}{color_reset}", 
                        i, 
                        left_ofst, 
                        prog_out_ln.line, 
                        mid_ofst, 
                        exp_out_ln.line);
                }

                // if !exp_out_ln.matched {
                //     mismatches += 1;
                // }
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

    // SUMMARY
    println!("\nSUMMARY:\n");
    if mismatches == 0 {
        println!("{color_green}Found 0 mismatch !");
    } else {
        if mismatches == 1 {
            println!("{color_red}Found 1 mismatch :");
        } else {
            println!("{color_red}Found {} mismatches :", mismatches);
        }


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
                                                                                // Maybe its better to have something like `let exp_out_t = string_to_out_t(exp_out)` ???
    let mut prog_out_t: OutputT = OutputT::new(prog_out);                           // or maybe OutputT { lines: Vec::new() }
    let mut exp_out_t: OutputT = OutputT::new(exp_out);
    
    //loop that performs the test
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

    print_result(prog_out_t, exp_out_t);
}
