use getopts::Options;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug)]
pub enum Quality {
    Lossy { quality: f32 },
    Lossless,
}

pub struct CliArgs {
    pub quality: Quality,
    pub files: Vec<(PathBuf, PathBuf)>,
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
        opts.optopt(
            "q",
            "quality",
            "Quality of WebP image: 0-100. Default 80.\nMutually exclusive with -l",
            "FLOAT",
        );
        opts.optflag(
            "l",
            "lossless",
            "Encode losslessly.\nMutually exclusive with -q",
        );
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{}\n{}", e, Self::usage(program, opts));
                return None;
            }
        };

        if matches.opt_present("lossless") && matches.opt_present("quality") {
            eprintln!(
                "towebp: -q and -l are mutually exclusive, you may only specify one at a time"
            );
            return None;
        }

        let quality = if matches.opt_present("lossless") {
            Quality::Lossless
        } else {
            let quality = match matches.opt_get_default("quality", 80f32) {
                Ok(q) => q,
                Err(_) => {
                    eprintln!(
                        "towebp: quality number was not understood. It must be from 0 to 100"
                    );
                    return None;
                }
            };

            Quality::Lossy { quality }
        };

        let mut files = vec![];
        for inputs in matches.free {
            let ipath = PathBuf::from(inputs);
            let opath = Self::calculate_opath(&ipath);
            files.push((ipath, opath));
        }

        Some(Self { quality, files })
    }

    fn calculate_opath(ipath: &PathBuf) -> PathBuf {
        let mut ret = ipath.clone();
        ret.set_extension("webp");

        ret
    }
}
