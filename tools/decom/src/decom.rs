#![feature(bufreader_peek)]
use std::{
    collections::HashMap,
    env::args,
    fs::{read_dir, OpenOptions},
    io::{BufReader, Write},
    path::PathBuf,
    str::FromStr,
    thread::spawn,
};

use anyhow::Result;
use bincode::decode_from_std_read;
use log::*;
use rfe::ToCsvClean;
use rfe::{msg::MsgPacket, BINCODE_CONFIG};
use simple_logger::SimpleLogger;

fn decom_file(file_path: String, out_dir: String) -> Result<()> {
    info!("decomming {file_path}");
    let f = OpenOptions::new().read(true).open(file_path)?;
    let mut rea = BufReader::new(f);
    let mut files = HashMap::new();

    'outer: loop {
        while let Ok(_) = rea.peek(1) {
            let msg = match decode_from_std_read::<MsgPacket, _, _>(&mut rea, BINCODE_CONFIG) {
                Ok(m) => m,
                Err(_) => break 'outer,
            };
            if !files.contains_key(&msg.msg.kind()) {
                let path = PathBuf::from_str(&out_dir)?
                    .join(format!("{:?}.csv", msg.msg.kind()).to_lowercase());
                let exists = path.exists();
                let mut w = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(path)?;

                if !exists {
                    let values = msg.to_csv_clean();
                    let mut line = values
                        .iter()
                        .map(|x| x.split_once(" = ").expect("unexpected csv data").0)
                        .collect::<Vec<&str>>()
                        .join(",");
                    line += "\n";
                    w.write(line.as_bytes())?;
                }

                files.insert(msg.msg.kind(), w);
            }

            if let Some(w) = files.get_mut(&msg.msg.kind()) {
                let values = msg.to_csv_clean();
                let mut line = values
                    .iter()
                    .map(|x| x.split_once(" = ").expect("unexpected csv data").1)
                    .collect::<Vec<&str>>()
                    .join(",");
                line += "\n";
                w.write(line.as_bytes())?;
            }
        }
    }
    return Ok(());
}

fn decom_task(folder: String, out_dir: String) -> Result<()> {
    let mut tasks = Vec::new();
    let mut files = Vec::new();
    for ent in read_dir(folder)? {
        let ent = ent?;

        if ent.metadata()?.is_dir() {
            let folder = ent.path().to_string_lossy().to_string();
            let out_dir = out_dir.clone();
            tasks.push(spawn(move || decom_task(folder, out_dir)));
        } else if ent.metadata()?.is_file() {
            let file_path = ent.path().to_string_lossy().to_string();
            files.push(file_path);
        }
    }

    files.sort();
    for file_path in files.iter() {
        let out_dir = out_dir.clone();
        if let Err(e) = decom_file(file_path.clone(), out_dir) {
            error!("{e}");
        }
    }

    for task in tasks {
        task.join().ok();
    }

    return Ok(());
}

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();
    info!("started decom");

    let args = args().skip(1).collect::<Vec<String>>();
    let folder = args[0].clone();
    let out_dir = args[1].clone();

    decom_task(folder, out_dir)?;

    info!("finished decom");
    return Ok(());
}
