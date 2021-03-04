use getopts::Options;
use std::path::PathBuf;

pub struct CliArgs {
	pub quality: f32,
	pub ipath: PathBuf,
	pub opath: PathBuf
}

impl CliArgs {
    pub fn new() -> Option<Self> {
        let cli = Self::cli();

        cli
    }

	fn usage(program: &str, opts: Options) -> String {
        let brief = format!("Usage: {} [options]", program);
        format!("{}", opts.usage(&brief))
    }

	fn cli() -> Option<Self> {
		let args: Vec<String> = std::env::args().collect();
		let program = &args[0];

		let mut opts = Options::new();
		opts.optopt("q", "quality", "Quality of WebP image: 0-100", "FLOAT");
		opts.reqopt("i", "ipath", "Path you want to convert to WebP", "PATH");
		opts.optopt("o", "opath", "Path you want the WebP to be placed", "PATH");
		let matches = match opts.parse(&args[1..]) {
			Ok(m) => m,
			Err(e) => {
				println!("{}\n{}", e, Self::usage(program, opts));
				return None;
			}
		};

		let quality = matches.opt_get_default("quality", 80f32).expect("Failed to parse quality");
		let ipath: PathBuf = PathBuf::from(matches.opt_str("ipath").expect("ipath is required, how did we get here?"));
		let opath: PathBuf = match matches.opt_str("opath").map(|s| PathBuf::from(s)) {
			Some(p) => p,
			None => Self::calculate_opath(&ipath)
		};

		Some(Self {
			quality,
			ipath,
			opath
		})
	}

	fn calculate_opath(ipath: &PathBuf) -> PathBuf {
		let mut ret = ipath.clone();
		ret.set_extension("webp");

		ret
	}
}