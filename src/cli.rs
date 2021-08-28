use std::env;
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub(super) struct Options {
    pub file1: String,
    pub file2: String,
    pub key1: String,
    pub key2: String,
    pub allow_no_key: bool,
    pub merge: Option<u8>,
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [OPTIONS] FILE1 FILE2", program);
    print!("{}", opts.usage(&brief));
}

pub(super) fn parse_opts() -> Options {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];
    let mut opts = getopts::Options::new();

    opts.optopt("k", "key", "JSON key to join", "KEY");
    opts.optopt("1", "key1", "JSON key to join of FILE1", "KEY1");
    opts.optopt("2", "key2", "JSON key to join of FILE2", "KEY2");
    opts.optopt(
        "m",
        "merge",
        "Merge the paired JSON",
        "PRIORITY_FILENUM (1 or 2)",
    );
    opts.optflag("", "allow-no-key", "Allow no key");
    opts.optflag("v", "version", "Print version and exit");
    opts.optflag("h", "help", "Print usage and exit");

    let matches = opts.parse(&args[1..]).unwrap();

    if args.len() == 1 || matches.opt_present("h") {
        print_usage(&program, opts);
        process::exit(0)
    }

    if matches.opt_present("v") {
        println!("{}", VERSION);
        process::exit(0)
    }

    let opt_key = matches.opt_str("k");
    let opt_key1 = matches.opt_str("1");
    let opt_key2 = matches.opt_str("2");

    if opt_key.is_none() && (opt_key1.is_none() || opt_key2.is_none()) {
        panic!("'-k' or '-1/-2' is required");
    }

    let (key1, key2) = if opt_key.is_some() {
        let key = opt_key.unwrap();
        (key.clone(), key)
    } else {
        (opt_key1.unwrap(), opt_key2.unwrap())
    };

    let merge = match matches.opt_str("m") {
        Some(n) => match &*n {
            "1" => Some(1),
            "2" => Some(2),
            _ => panic!("Specify 1 or 2 for '-m'"),
        },
        None => None,
    };

    let allow_no_key = matches.opt_present("allow-no-key");

    let (file1, file2) = match matches.free.len() {
        2 => (matches.free[0].to_string(), matches.free[1].to_string()),
        _ => {
            print_usage(&program, opts);
            process::exit(1)
        }
    };

    Options {
        file1: file1,
        file2: file2,
        key1: key1,
        key2: key2,
        allow_no_key: allow_no_key,
        merge: merge,
    }
}
