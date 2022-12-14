use sais::sais32::*;
use std::io::{self, Read};
use std::time::Instant;

use clap::Parser;

const ROUND: usize = 3;
const PARALLEL: bool = false;
const FREE_SPACE: usize = 6 * 1024;
const LCP: bool = false;
const VALIDATE: bool = false;

/// Run 32-bit sais (optionally with plcp/lcp) on samples
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cmdline {
    /// Number of times to run on each sample
    #[clap(short, long, value_parser, default_value_t = ROUND)]
    pub round: usize,

    /// Whether to run in parallel
    #[clap(short, long, value_parser, default_value_t = PARALLEL)]
    pub parallel: bool,

    /// Suggested free space after allocated suffix array
    #[clap(short, long, value_parser, default_value_t = FREE_SPACE)]
    pub free_space: usize,

    /// Whether to compute plcp array and lcp array
    #[clap(short, long, value_parser, default_value_t = LCP)]
    pub lcp: bool,

    /// Whether to validate suffix array and lcp array
    #[clap(short, long, value_parser, default_value_t = VALIDATE)]
    pub validate: bool,

    /// Sample files to run
    #[clap(value_parser, value_name = "SAMPLE")]
    pub samples: Vec<String>,
}

fn main() {
    let cmdline = Cmdline::parse();
    for sample in cmdline.samples.iter() {
        if let Err(err) = run(sample.as_str(), cmdline.round, cmdline.parallel, cmdline.free_space, cmdline.lcp, cmdline.validate) {
            eprintln!("error: {:?}", err);
            println!();
        }
    }
}

fn run(filename: &str, round: usize, enable_parallel: bool, free_space: usize, enable_lcp: bool, enable_validate: bool) -> io::Result<()> {
    println!("*** run 32-bit sais on sample file {:?} ***", filename);

    println!("> load file...");
    let mut file = std::fs::File::open(filename)?;
    let mut text = Vec::new();
    let size = file.read_to_end(&mut text)?;
    println!("  file size is {} bytes", size);
    if size > MAX_LENGTH {
        Err(error("sample file too large"))?;
    }

    println!("> allocate suffix array...");
    let mut suffix_array = vec![0; Ord::min(size.saturating_add(free_space), MAX_LENGTH)];
    let actual_free_space = suffix_array.len() - size;
    println!("  allocated suffix array of {} + {} = {} 32-bit words", size, actual_free_space, suffix_array.len());

    let mut plcp_array = Vec::new();
    let mut lcp_array = Vec::new();
    if enable_lcp {
        println!("> allocate plcp array and lcp array...");
        plcp_array = vec![0; size];
        lcp_array = vec![0; size];
    }

    println!("> allocate sais context...");
    let mut context = (if enable_parallel {
        println!("  multiple threaded context (openmp default thread count)");
        SaisContext::new_parallel(0)
    } else {
        println!("  singled threaded context");
        SaisContext::new()
    })
    .ok_or_else(|| error("unable to allocate sais context"))?;

    for number in 1..=round {
        println!("> compute suffix array (optionally with plcp/lcp), round {}...", number);

        let sais_start = Instant::now();
        context
            .sais(&text[..], &mut suffix_array[..], None)
            .map_err(|err| error(format!("sais error: {:?}", err)))?;
        let sais_elapsed = sais_start.elapsed();
        println!("  suffix array computed in {:.3}s", sais_elapsed.as_secs_f64());

        if enable_validate {
            println!("    validating suffix array...");
            let suffix_array_ok = validate_suffix_array(&text[..], &suffix_array[..]);
            println!("    suffix array validation: {}", if suffix_array_ok { "ok" } else { "failed" });
            if !suffix_array_ok {
                Err(error("suffix array validation failed"))?;
            }
        }

        if enable_lcp {
            let plcp_start = Instant::now();
            (if enable_parallel {
                parallel::plcp(&text[..], &suffix_array[..size], &mut plcp_array[..], 0)
            } else {
                plcp(&text[..], &suffix_array[..size], &mut plcp_array[..])
            })
            .map_err(|err| error(format!("plcp error: {:?}", err)))?;
            let plcp_elapsed = plcp_start.elapsed();
            println!("  plcp array computed in {:.3}s", plcp_elapsed.as_secs_f64());

            let lcp_start = Instant::now();
            (if enable_parallel {
                parallel::lcp(&plcp_array[..], &suffix_array[..size], &mut lcp_array[..], 0)
            } else {
                lcp(&plcp_array[..], &suffix_array[..size], &mut lcp_array[..])
            })
            .map_err(|err| error(format!("lcp error: {:?}", err)))?;
            let lcp_elapsed = lcp_start.elapsed();
            println!("  lcp array computed in {:.3}s", lcp_elapsed.as_secs_f64());

            if enable_validate {
                println!("    validating lcp array...");
                let lcp_array_ok = validate_lcp_array(&text[..], &suffix_array[..], &lcp_array[..]);
                println!("    lcp array validation: {}", if lcp_array_ok { "ok" } else { "failed" });
                if !lcp_array_ok {
                    Err(error("lcp array validation failed"))?;
                }
            }

            println!("  total {:.3}s", (sais_elapsed + plcp_elapsed + lcp_elapsed).as_secs_f64());
        }
    }

    println!();
    Ok(())
}

fn validate_suffix_array(text: &[u8], suffix_array: &[i32]) -> bool {
    if text.len() > 0 {
        if suffix_array.len() < text.len() {
            return false;
        }
        for i in 0..text.len() - 1 {
            if text[suffix_array[i] as usize..] >= text[suffix_array[i + 1] as usize..] {
                return false;
            }
        }
    }
    true
}

fn validate_lcp_array(text: &[u8], suffix_array: &[i32], lcp_array: &[i32]) -> bool {
    if text.len() > 0 {
        if (suffix_array.len() < text.len()) || (lcp_array.len() < text.len()) || lcp_array[0] != 0 {
            return false;
        }
        for i in 1..text.len() {
            let common = Iterator::zip(text[suffix_array[i - 1] as usize..].iter(), text[suffix_array[i] as usize..].iter())
                .take_while(|(&x, &y)| x == y)
                .count();
            if lcp_array[i] != (common as i32) {
                return false;
            }
        }
    }
    true
}

fn error<S: Into<String>>(message: S) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message.into())
}
