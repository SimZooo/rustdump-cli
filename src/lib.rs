use std::{io, path::PathBuf};
use core::{create_dump, OutputRow};

pub fn dump(path: PathBuf, line_length: Option<usize>) {
    let mut out = vec![];
    let mut last_row: Option<OutputRow> = None;
    let output = create_dump(path);
    for row in output {
        // "Concatenate" rows with identical bytes
        if let Some(last_row_some) = last_row && last_row_some.bytes == row.bytes {
            let last = out.last();
            if let Some(last) = last {
                if *last == "*" {
                    out.push(String::from("*"));
                }
            }
            last_row = Some(row);
            continue;
        }
        out.push(row.to_string());

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
        //println!("{}", out.join("\n"));
    }
}
