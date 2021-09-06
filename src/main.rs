use pow::*;
use std::io;
use std::sync::mpsc;
use std::thread;
const DIFFICULTY: [u8; 3] = [0, 0, 0];

fn slave(mut message: Message, length: u128) -> Result<Message, ()> {
    for _ in 0..length {
        if !message.hash().starts_with(&DIFFICULTY) {
            message.update();
        } else {
            return Ok(message);
        }
    }
    Err(())
}

fn main() -> io::Result<()> {
    println!("Please input your message:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

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
            println!("{:?}", m);
            println!("{:?}", m.hash());
            return Ok(());
        }
    }
    Ok(())
}
