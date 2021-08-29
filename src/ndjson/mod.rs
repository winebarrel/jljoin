#[cfg(test)]
mod tests;

use anyhow::anyhow;
use anyhow::Result;
use serde_json::json;
use serde_json::Value;
use std::fs;
use std::io;
use std::io::Seek;

pub(super) struct Opts {
    pub allow_no_key: bool,
    pub merge: Option<u8>,
}

pub(super) fn join<T>(
    file1: fs::File,
    key1: &str,
    file2: fs::File,
    key2: &str,
    fout: T,
    opts: Opts,
) -> Result<()>
where
    T: io::Write,
{
    let mut reader1 = io::BufReader::new(file1);
    let mut reader2 = io::BufReader::new(file2);
    let mut writer = io::BufWriter::new(fout);

    let mut prev1 = None;
    let mut block_start: u64 = 0;
    let mut block_end: u64 = 0;

    loop {
        let curt1 = read_line_with_parsing(&mut reader1)?;

        if curt1.is_none() {
            break;
        }

        let json1 = curt1.as_ref().unwrap();
        let val1 = json_get_or_err(json1, key1, opts.allow_no_key)?;

        if let Some(ref prev_json1) = prev1 {
            let prev_val1 = json_get_or_err(prev_json1, key1, opts.allow_no_key)?;

            if val1 != prev_val1 {
                // Go to the next block in NDJSON2
                reader2.seek(io::SeekFrom::Start(block_end))?;
                block_start = block_end
            } else {
                // Repeat the current block in NDJSON2
                reader2.seek(io::SeekFrom::Start(block_start))?;
            }
        }

        let mut prev2 = None;

        loop {
            block_end = reader2.stream_position()?;
            let curt2 = read_line_with_parsing(&mut reader2)?;

            if curt2.is_none() {
                break;
            }

            let json2 = curt2.as_ref().unwrap();
            let val2 = json_get_or_err(json2, key2, opts.allow_no_key)?;

            if let Some(ref prev_json2) = prev2 {
                let prev_val2 = json_get_or_err(prev_json2, key2, opts.allow_no_key)?;

                if val2 != prev_val2 {
                    break;
                }
            }

            if val1 == val2 {
                print_pair(&mut writer, &json1, json2, opts.merge)?;
            }

            prev2 = curt2;
        }

        prev1 = curt1;
    }

    Ok(())
}

fn read_line_with_parsing<T>(reader: &mut T) -> Result<Option<Value>>
where
    T: io::BufRead,
{
    let mut line = String::new();
    let n = reader.read_line(&mut line)?;

    if n == 0 {
        return Ok(None);
    }

    let r: serde_json::error::Result<Value> = serde_json::from_str(&line);

    match r {
        Err(e) => {
            let ctx = format!("Failed to parse JSON: {}", &line);
            Err(anyhow::Error::new(e).context(ctx))
        }
        Ok(v) => {
            if !v.is_object() {
                Err(anyhow!("JSON in row is not Object type: {}", v))
            } else {
                Ok(Some(v))
            }
        }
    }
}

// NOTE: Copy from https://github.com/serde-rs/json/issues/377#issuecomment-341490464
fn merge_obj(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge_obj(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

fn print_pair<T>(writer: &mut T, json1: &Value, json2: &Value, merge: Option<u8>) -> io::Result<()>
where
    T: io::Write,
{
    let line = if let Some(n) = merge {
        assert!(n == 1 || n == 2);

        if n == 1 {
            let mut out_json = json2.clone();
            merge_obj(&mut out_json, &json1);
            out_json
        } else {
            let mut out_json = json1.clone();
            merge_obj(&mut out_json, &json2);
            out_json
        }
        .to_string()
    } else {
        format!("[{},{}]", json1, json2)
    };

    writeln!(writer, "{}", line)
}

fn json_get_or_err(json: &Value, key: &str, allow_no_key: bool) -> Result<Value> {
    if let Some(v) = json.get(key) {
        return Ok(v.clone());
    }

    if allow_no_key {
        return Ok(json!(null));
    }

    Err(anyhow!("Key '{}' does not exist: {}", key, json))
}
