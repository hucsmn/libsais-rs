use sais::sais32::*;
use std::io::{self, Read};
use std::time::Instant;

use clap::Parser;

const ROUND: usize = 3;
const PARALLEL: bool = false;
const FREE_SPACE: usize = 6 * 1024;
const VALIDATE: bool = false;

/// Run 32-bit bwt/unbwt on samples
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cmdline {
    /// Number of times to run on each sample
    #[clap(short, long, value_parser, default_value_t = ROUND)]
    pub round: usize,

    /// Whether to run in parallel
    #[clap(short, long, value_parser, default_value_t = PARALLEL)]
    pub parallel: bool,

    /// Suggested free space after allocated temporary array (>= 1)
    #[clap(short, long, value_parser, default_value_t = FREE_SPACE)]
    pub free_space: usize,

    /// Whether to validate result
    #[clap(short, long, value_parser, default_value_t = VALIDATE)]
    pub validate: bool,

    /// Sample files to run
    #[clap(value_parser, value_name = "SAMPLE")]
    pub samples: Vec<String>,
}

fn main() {
    let cmdline = Cmdline::parse();
    for sample in cmdline.samples.iter() {
        if let Err(err) = run(sample.as_str(), cmdline.round, cmdline.parallel, cmdline.free_space, cmdline.validate) {
            eprintln!("error: {:?}", err);
            println!();
        }
    }
}

fn run(filename: &str, round: usize, enable_parallel: bool, free_space: usize, enable_validate: bool) -> io::Result<()> {
    println!("*** run 32-bit bwt/unbwt on sample file {:?} ***", filename);

    println!("> load file...");
    let mut file = std::fs::File::open(filename)?;
    let mut text = Vec::new();
    let size = file.read_to_end(&mut text)?;
    println!("  file size is {} bytes", size);
    if size > MAX_LENGTH {
        Err(error("sample file too large"))?;
    }

    println!("> allocate bwt text and unbwt text...");
    let mut bwt_text = vec![0; size];
    let mut unbwt_text = vec![0; size];

    println!("> allocate temporary array...");
    let mut temporary_array = vec![0; Ord::min(size.saturating_add(Ord::max(free_space, 1)), MAX_LENGTH)];
    let actual_free_space = temporary_array.len() - size;
    println!("  allocated temporary array of {} + {} = {} 32-bit words", size, actual_free_space, temporary_array.len());
    if actual_free_space == 0 {
        Err(error("unbwt requires: temporary_array.len() >= text.len() + 1"))?
    }

    println!("> allocate bwt context and unbwt context...");
    let (mut bwt_context, mut unbwt_context) = (if enable_parallel {
        println!("  multiple threaded context (openmp default thread count)");
        Option::zip(SaisContext::new_parallel(0), UnbwtContext::new_parallel(0))
    } else {
        println!("  singled threaded context");
        Option::zip(SaisContext::new(), UnbwtContext::new())
    })
    .ok_or_else(|| error("unable to allocate context"))?;

    for number in 1..=round {
        println!("> compute bwt and unbwt, round {}...", number);

        let start_bwt = Instant::now();
        let primary_index = bwt_context
            .bwt(&text[..], &mut bwt_text[..], &mut temporary_array[..], None)
            .map_err(|err| error(format!("bwt error: {:?}", err)))?;
        let elapsed_bwt = start_bwt.elapsed();
        println!("  bwt computed in {:.3}s", elapsed_bwt.as_secs_f64());

        let start_unbwt = Instant::now();
        unbwt_context
            .unbwt(&bwt_text[..], &mut unbwt_text[..], &mut temporary_array[..], None, primary_index)
            .map_err(|err| error(format!("unbwt error: {:?}", err)))?;
        let elapsed_unbwt = start_unbwt.elapsed();
        println!("  unbwt computed in {:.3}s", elapsed_unbwt.as_secs_f64());

        if enable_validate {
            println!("    validating bwt/unbwt...");
            let bwt_unbwt_ok = text == unbwt_text;
            println!("    validate bwt/unbwt: {}", if bwt_unbwt_ok { "ok" } else { "failed" });
            if !bwt_unbwt_ok {
                Err(error("bwt/unbwt validation failed"))?;
            }
        }
    }

    println!();
    Ok(())
}

fn error<S: Into<String>>(message: S) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message.into())
}
