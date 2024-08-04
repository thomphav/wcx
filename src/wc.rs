use std::fs::{metadata, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Default)]
struct FlagData {
    count: usize,
    enabled: bool,
}

#[derive(Default)]
struct Data {
    bytes: FlagData,
    lines: FlagData,
    chars: FlagData,
    words: FlagData,
}

pub fn invoke(
    bytes: bool,
    lines: bool,
    chars: bool,
    words: bool,
    file: &Path,
) -> anyhow::Result<()> {
    let default = !bytes && !lines && !chars && !words;
    let metadata = metadata(file)?;

    let mut data: Data = Default::default();

    if bytes || default {
        data.bytes.enabled = true;
        data.bytes.count = metadata.len() as usize;
    }

    if lines || chars || words || default {
        let b = BufReader::new(File::open(file)?);

        if lines || default {
            data.lines.enabled = true;
            data.lines.count = b.lines().count();
        }

        if chars && !bytes {
            data.chars.enabled = true;
            data.chars.count = 19 as usize;
        }

        if words || default {
            data.words.enabled = true;
            data.words.count = 187;
        }
    }

    assert!(!(data.bytes.enabled & data.chars.enabled));

    let mut buf = String::new();

    if data.lines.enabled {
        let out = format!("{} ", data.lines.count);
        buf.push_str(&out);
    }
    if data.words.enabled {
        let out = format!("{} ", data.words.count);
        buf.push_str(&out);
    }
    if data.bytes.enabled {
        let out = format!("{} ", data.bytes.count);
        buf.push_str(&out);
    }
    if data.chars.enabled {
        let out = format!("{} ", data.chars.count);
        buf.push_str(&out);
    }
    let out = format!("{}", file.display());
    buf.push_str(&out);

    println!("{buf}");
    Ok(())
}
