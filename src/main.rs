#[macro_use]
extern crate clap;
use pow::*;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;

fn main() -> io::Result<()> {
    // configure command line args
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "jasex")
        (about: "Filter e-mail with POW")
        (@arg MODE: -m --mode +required +takes_value "Select working mode.")
        (@arg FILE: -f --file +takes_value "Choose file to read")
        (@arg OUT: -o --out +takes_value "Output file")
    )
    .get_matches();

    if matches.value_of("MODE").unwrap() == "g" {
        //generate mode
        let mut input = String::new();
        if let Some(s) = matches.value_of("FILE") {
            let mut f = File::open(s)?;
            f.read_to_string(&mut input)?;
        } else {
            println!("Please input your message:");
            io::stdin().read_line(&mut input)?;
        }

        let mut message = Message::new(input);
        let thread_num = 4;
        let offset = u128::MAX / thread_num;
        let (tx, rx) = mpsc::channel();

        // start working threads
        for _ in 0..thread_num {
            let txn = mpsc::Sender::clone(&tx);
            let msg = message.clone();
            message.nounce += offset;
            thread::spawn(move || {
                let m = slave(msg, offset);
                txn.send(m).unwrap();
            });
        }

        // message comes
        for out in rx.recv() {
            // if any lucky dog finds out!
            if let Ok(m) = out {
                if let Some(path) = matches.value_of("OUT") {
                    if let Ok(mut f) = File::open(path) {
                        f.write(serde_json::to_string(&m).unwrap().as_bytes());
                    } else {
                        if let Ok(mut f) = File::create(path) {
                            f.write(serde_json::to_string(&m).unwrap().as_bytes());
                        } else {
                            println!("Error creating file");
                            std::process::exit(1);
                        }
                    }
                } else {
                    println!("{}", serde_json::to_string(&m).unwrap());
                    println!("{:?}", m.hash());
                }
                return Ok(());
            }
        }
    } else if matches.value_of("MODE").unwrap() == "v" {
        // verify mode
        let mut json = String::new();
        if let Some(s) = matches.value_of("FILE") {
            let mut f = File::open(s)?;
            f.read_to_string(&mut json)?;
        } else {
            println!("Please input the json:");
            io::stdin().read_line(&mut json)?;
        }

        // parse the json and verify
        let msg: Message = serde_json::from_str(json.trim()).expect("Parsing error");
        if msg.hash().starts_with(&DIFFICULTY) {
            println!("OK");
        } else {
            println!("Not right");
            std::process::exit(1);
        }
    }

    Ok(())
}
