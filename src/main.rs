mod cli;
mod ndjson;

use std::fs;
use std::io;

fn main() {
    let opts = cli::parse_opts();

    let f1 = fs::File::open(opts.file1).unwrap();
    let f2 = fs::File::open(opts.file2).unwrap();

    ndjson::join(
        f1,
        &opts.key1,
        f2,
        &opts.key2,
        io::stdout(),
        ndjson::Opts {
            allow_no_key: opts.allow_no_key,
            merge: opts.merge,
        },
    )
    .unwrap();
}
