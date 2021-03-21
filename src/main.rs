use std::collections::HashMap;
use std::fs::File;
use std::io::stdin;
use std::io::stdout;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::process::exit;
use structopt::StructOpt;

fn main() {
	let args: Args = Args::from_args();
	if args.debug {
		dbg!(&args);
	}

	if args.ignore_char_count > 0 || args.ignore_field_count > 0 {
		eprintln!("-f and -s are not supported yet. Sorry!");
		exit(2);
	}

	let input: Box<dyn BufRead> = match args.input_file.filter(|it| it != "-") {
		None => Box::new(BufReader::new(stdin())),
		Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
	};
	let mut output: Box<dyn Write> = match args.output_file {
		None => Box::new(BufWriter::new(stdout())),
		Some(filename) => Box::new(BufWriter::new(File::open(filename).unwrap())),
	};

	let mut counts: HashMap<String, u32> = HashMap::new();
	let mut last_height = 0;
	for line in input.lines() {
		let line = line.unwrap();
		let line = if args.case_insensitive {
			line.to_lowercase()
		} else {
			line
		};
		if args.debug {
			dbg!(&line);
		}

		*counts.entry(line).or_insert(0) += 1;

		if !args.debug {
			// For each previous line in the output, move the cursor up and clear the line.
			for _ in 0..last_height {
				output.write_all("\u{001B}[F\u{001B}[K".as_ref()).unwrap();
			}
		}

		let mut pairs: Vec<(&String, &u32)> = counts.iter().collect();
		pairs.sort_by(|a, b| a.1.cmp(b.1).reverse());

		for (line, count) in pairs {
			output
				.write_all(format!("{}\t{}\n", count, line).as_ref())
				.unwrap();
		}
		output.flush().unwrap();

		last_height = counts.len();
	}
}

#[derive(Debug, StructOpt)]
struct Args {
	/// Case insensitive comparison of lines.
	#[structopt(short = "i")]
	case_insensitive: bool,

	/// Ignore the first num fields in each input line when doing comparisons. A field is a string of
	/// non-blank characters separated from adjacent fields by blanks. Field numbers are one based,
	/// i.e., the first field is field one.
	#[structopt(short = "f", default_value = "0", name = "num")]
	ignore_field_count: u32,

	/// Ignore the first chars characters in each input line when doing comparisons. If specified in
	/// conjunction with the -f option, the first chars characters after the first num fields will be
	/// ignored.  Character numbers are one based, i.e., the first character is character one.
	#[structopt(short = "s", default_value = "0", name = "chars")]
	ignore_char_count: u32,

	/// The input file to read, or "-" indicating to read stdin. If omitted, stdin will be used.
	input_file: Option<String>,

	/// The output file to write. If omitted, stdout will be used.
	output_file: Option<String>,

	#[structopt(long, hidden = true)]
	debug: bool,
}
