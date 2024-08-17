use std::{sync::mpsc, thread, time::Duration};

use anyhow::{anyhow, Result};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

impl Msg {
    fn new(id: usize, value: usize) -> Self {
        Self { idx: id, value }
    }
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        // random exit the producer
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exiting", idx);
            break;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // create producers
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx); // drop tx, otherwise the receiver will wait forever

    // create consumer
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer: exiting");
        42
    });

    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;
    println!("secret: {}", secret);
    Ok(())
}
