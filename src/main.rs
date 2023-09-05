/*
    Ref 1. https://github.com/tokio-rs/tokio/discussions/2648
    Ref 2. https://stackoverflow.com/a/67650171
    Ref 3. https://stackoverflow.com/a/73336289
 */

use std::{collections::HashMap, error::Error, time, thread, sync::Arc};

use futures::stream::FuturesUnordered;
use tokio::sync::Semaphore;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let mut map: HashMap<String, String> = HashMap::<String, String>::with_capacity(10);

    map.insert("1".to_string(), "1".to_string());
    map.insert("2".to_string(), "2".to_string());
    map.insert("3".to_string(), "3".to_string());
    map.insert("4".to_string(), "4".to_string());
    println!("{}", map.len());

    let max_concurrent: usize= 1;
    let async_pool = Arc::new(Semaphore::new(max_concurrent));

    let tasks = map
    .iter()
    .map(|(key, val)| {
        let key = key.clone();
        let val = val.clone();
        let accuire_pool = Arc::clone(&async_pool).acquire_owned();

        tokio::spawn(async move {
            let _release_pool = accuire_pool.await;
            println!("key: {key} val: {val}");
            let ten_millis = time::Duration::from_millis(500);
            thread::sleep(ten_millis);
        })
    }).collect::<FuturesUnordered<_>>();

    let errgroup = futures::future::join_all(tasks).await;
    println!("{:?}", errgroup);

    Ok(())
}
