use getopts::Options;
use std::path::PathBuf;

pub struct CliArgs {
	pub quality: f32,
	pub files: Vec<(PathBuf, PathBuf)>
}

impl CliArgs {
    pub fn new() -> Option<Self> {
        let cli = Self::cli();

        cli
    }

	fn usage(program: &str, opts: Options) -> String {
        let brief = format!("Usage: {} [options] FILE...", program);
        format!("{}", opts.usage(&brief))
    }

	fn cli() -> Option<Self> {
		let args: Vec<String> = std::env::args().collect();
		let program = &args[0];

		let mut opts = Options::new();
		opts.optopt("q", "quality", "Quality of WebP image: 0-100", "FLOAT");
		let matches = match opts.parse(&args[1..]) {
			Ok(m) => m,
			Err(e) => {
				println!("{}\n{}", e, Self::usage(program, opts));
				return None;
			}
		};

		let quality = matches.opt_get_default("quality", 80f32).expect("Failed to parse quality");
		
		let mut files = vec![];
		for inputs in matches.free {
			let ipath = PathBuf::from(inputs);
			let opath = Self::calculate_opath(&ipath);
			files.push((ipath, opath));
		}

		Some(Self {
			quality,
			files
		})
	}

	fn calculate_opath(ipath: &PathBuf) -> PathBuf {
		let mut ret = ipath.clone();
		ret.set_extension("webp");

		ret
	}
}