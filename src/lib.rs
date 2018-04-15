use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

const MIN_LINE_WIDTH: usize = 50;
const DEFAULT_LINE_WIDTH: usize = 80;

pub struct Config {
    pub in_filename: String,
    pub out_filename: String,
    pub line_width: usize,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // Skip program name
        args.next();
        let in_filename = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a bin to dump.")
        };
        let out_filename = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a name for the output file.")
        };
        let line_width = match args.next() {
            Some(arg) => {
                let line_width = arg.parse::<usize>().unwrap();
                if line_width < MIN_LINE_WIDTH {
                    return Err("line width too small, try bigger.");
                }
                line_width
            }
            None => DEFAULT_LINE_WIDTH
        };
        Ok(Config { in_filename, out_filename, line_width })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    // Open input file and process it
    let in_f = File::open(config.in_filename)?;
    let processed_string = process_bin_file(in_f, config.line_width)?;
    // Create output file and write it
    let mut out_f = File::create(config.out_filename)?;
    out_f.write_all(processed_string.as_bytes())?;
    Ok(())
}

fn process_bin_file(bin_file : File, line_width: usize) -> Result<String, Box<Error>> {
    let mut line_len = line_width + 1;
    let mut binarray = String::from("static const char[] BINARRAY = {");
    for byte in bin_file.bytes() {
        let array_item = format!("0x{:02x},", byte.unwrap());
        line_len += array_item.len();
        if line_len >= line_width {
            binarray.push_str("\n    ");
            line_len = array_item.len() + 4;
        }
        else {
            binarray.push_str(" ");
            line_len += 1;
        }
        binarray.push_str(&*array_item);
    }
    binarray.push_str("}");
    Ok(binarray)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn linewidth() {
        let test_f = File::open("src/testfiles/file1.bin").unwrap();
        let line_width = 50;
        let mut line_width_success = true;
        let generated_string = process_bin_file(test_f, line_width).unwrap();
        for line in generated_string.lines() {
            if line.len() > line_width {
                line_width_success = false;
                break;
            }
        }
        assert_eq!(line_width_success, true);
    }
}
