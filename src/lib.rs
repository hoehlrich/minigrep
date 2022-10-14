use clap::Parser;
use regex::Regex;
use std::error::Error;
use std::fs;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // Search each file
    for file in &args.files {
        // Add file prefix if needed (when more than one file)
        let mut out = "".to_string();
        if &args.files.len() > &1 {
            out = format!("{}:", &file);
        }

        for result in search_file(&args, file.to_string())? {
            println!("{}{}", out, result);
        }
    }
    Ok(())
}

/// Minimal grep rewritten in rust... minigrep!
#[derive(Parser, Debug)]
#[clap(author, version, about, trailing_var_arg = true)]
pub struct Args {
    /// Pattern to search against
    #[clap(required=true)]
    pattern: String,

    /// Files to search
    #[arg(required=true)]
    files: Vec<String>,

    /// Use <PATTERNS> as the patterns. If used multiple times, search all patterns given
    #[arg(long="regexp", short='e', group="Matching Control")]
    patterns: Vec<String>,

    /// Ignore case distinctions in patterns and input data
    #[arg(short, long, group="Matching Control")]
    ignore_case: bool,

    /// Invert matching, to select non-matching lines
    #[arg(short='v', long, group="Matching Control")]
    invert_match: bool,

    /// Select only lines containing matches that for whole words
    #[arg(short='w', long, group="Matching Control")]
    word_regexp: bool,

    /// Select only matches that match the entire line
    #[arg(short='x', long, group="Matching Control")]
    line_regexp: bool,
}

impl Args {
    fn get_regexes(&self) -> Result<Vec<Regex>, regex::Error> {
        let patterns = match self.patterns.len(){
            0 => vec![self.pattern.clone()],
            _ => {
                let mut a: Vec<String> = self.patterns.clone();
                a.push(self.pattern.clone());
                a
            },
        };

        let mut regexes = vec![];

        for pattern in patterns {
            // Handle case sensitivity
            let pattern = match self.ignore_case {
                true => pattern.to_lowercase(),
                false => pattern.clone(),
            };

            // Handle matching control
            if self.word_regexp {
                regexes.push(Regex::new(&format!(r"\b({})\b", pattern))?);
            } else if self.line_regexp {
                regexes.push(Regex::new(&format!(r"^{}$", pattern))?);
            } else {
                regexes.push(Regex::new(&pattern)?);
            }
        }

        Ok(regexes)
    }
}

fn search_file(args: &Args, file: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut results = vec![];

    let contents = fs::read_to_string(file)?;
    let patterns = args.get_regexes()?;

    for line in contents.lines() {
        // Handle case sensitivity
        let match_line = match args.ignore_case {
            true => line.clone().to_lowercase(),
            false => line.clone().to_string(),
        };

        // Check if match
        // Handle invert search
        for pattern in patterns.iter() {
            if pattern.is_match(&match_line) != args.invert_match {
                results.push(line.to_string());
            }
        }
    }
    Ok(results)
}
