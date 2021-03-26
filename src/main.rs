#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::error::Error;
use std::fs::File;
use std::io::stdin;
use std::io::stdout;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

use structopt::StructOpt;

mod counter;
use counter::Config as CounterConfig;
use counter::Counter;

mod output;
use output::Config as OutputConfig;
use output::Output;

fn main() -> Result<(), Box<dyn Error>> {
	let args: Args = Args::from_args();
	if args.debug {
		dbg!(&args);
	}

	let counter_config = CounterConfig {
		case_insensitive: args.case_insensitive,
		ignore_field_count: args.ignore_field_count,
		ignore_char_count: args.ignore_char_count,
	};
	let mut counter = Counter::new(counter_config);

	let input: Box<dyn BufRead> = match args.input_file.as_deref() {
		None | Some("-") => Box::new(BufReader::new(stdin())),
		Some(filename) => Box::new(BufReader::new(File::open(filename)?)),
	};

	let output: Box<dyn Write> = match args.output_file {
		None => Box::new(BufWriter::new(stdout())),
		Some(filename) => Box::new(BufWriter::new(File::open(filename)?)),
	};
	let output_config = OutputConfig {
		reverse: args.reverse,
		head: args.limit,
		debug: args.debug,
	};
	let mut output = Output::new(output, output_config);

	for line in input.lines() {
		let line = line?;
		if args.debug {
			dbg!(&line);
		}

		counter.count(&line);
		output.print(&counter.counts)?;
	}

	Ok(())
}

/// Replicating the behavior of `sort | uniq -c | sort -nr` with output that updates
/// in real-time as each line is parsed.
#[derive(Debug, StructOpt)]
struct Args {
	/// Case insensitive comparison of lines.
	#[structopt(short = "i")]
	case_insensitive: bool,

	/// Ignore the first num fields in each input line when doing comparisons. A field is a string of
	/// non-blank characters separated from adjacent fields by blanks. Field numbers are one based,
	/// i.e., the first field is field one.
	#[structopt(short = "f", default_value = "0", name = "num")]
	ignore_field_count: usize,

	/// Ignore the first chars characters in each input line when doing comparisons. If specified in
	/// conjunction with the -f option, the first chars characters after the first num fields will be
	/// ignored.  Character numbers are one based, i.e., the first character is character one.
	#[structopt(short = "s", default_value = "0", name = "chars")]
	ignore_char_count: usize,

	/// Displays the first count lines of output.
	#[structopt(short, long, name = "count")]
	limit: Option<usize>,

	/// Reverse the output order showing items with the fewest counts at the top.
	#[structopt(short, long)]
	reverse: bool,

	/// The input file to read, or "-" indicating to read stdin. If omitted, stdin will be used.
	input_file: Option<String>,

	/// The output file to write. If omitted, stdout will be used.
	output_file: Option<String>,

	#[structopt(long, hidden = true)]
	debug: bool,
}
