use core::{OutputRow, create_dump};
use std::time::Instant;
use std::{io, path::PathBuf};

pub fn dump(path: PathBuf, line_length: Option<usize>) {
    let mut out = vec![];
    let mut last_row: Option<OutputRow> = None;
    let mut output = Vec::new();
    let start = Instant::now();
    let n = create_dump(path, &mut output);
    println!("Dumped {} bytes in {:.2?}", n, start.elapsed());
    for row in output {
        // "Concatenate" rows with identical bytes
        if let Some(last_row_some) = last_row
            && last_row_some.bytes == row.bytes
        {
            let last = out.last();
            if let Some(last) = last {
                if *last == "*" {
                    out.push(String::from("*"));
                }
            }
            last_row = Some(row);
            continue;
        }
        println!("{}", row.to_string());

        last_row = Some(row);
    }

    if let Some(line_length) = line_length {
        let mut offset = 0;
        loop {
            let out = (offset..offset + line_length)
                .filter_map(|i| {
                    if i < out.len() {
                        Some(out[i].to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>();
            if offset >= out.len() {
                break;
            }
            offset += line_length;
            println!("{}", out.join("\n"));
            println!(
                "Press Enter to continue ({}/{})...",
                (offset / line_length) as usize,
                (out.len() / line_length) as usize
            );
            let mut t = String::new();
            let _ = io::stdin().read_line(&mut t);
        }

        return;
    } else {
    }
}
