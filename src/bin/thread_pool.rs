use std::{thread, time::Duration, sync::mpsc};

use rayon::ThreadPoolBuilder;



fn job(id: i32) -> i32{
    println!("start job {}", id);
    thread::sleep(Duration::from_secs(3));
    println!("finish job {}", id);
    id * 2
}

fn main() {
    let pool = ThreadPoolBuilder::new().num_threads(3).build().unwrap();
    let (tx, rx) = mpsc::channel();
    let params = vec![1,2,3,4,5,6,7,8,9,10,11,12];
    for param in params {
        let tx = tx.clone();
        pool.spawn(move ||tx.send(job(param)).unwrap());
    }
    drop(tx);
    let results: Vec<i32> = rx.into_iter().collect();
    dbg!(results);
}