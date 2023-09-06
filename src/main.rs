/*
   Ref 1. https://github.com/tokio-rs/tokio/discussions/2648
   Ref 2. https://stackoverflow.com/a/67650171
   Ref 3. https://stackoverflow.com/a/73336289
*/

use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};

use futures::stream::FuturesUnordered;
use tokio::{sync::Semaphore, time::sleep};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let mut map: HashMap<String, String> = HashMap::<String, String>::with_capacity(10);

    for i in 0..9999 {
        map.insert(i.to_string(), i.to_string());
    }

    println!("{}", map.len());

    let max_concurrent: usize = 999;
    let async_pool = Arc::new(Semaphore::new(max_concurrent));

    let tasks = map
        .iter()
        .map(|(key, val)| {
            let key = key.clone();
            let val = val.clone();
            let acquire_pool = Arc::clone(&async_pool).acquire_owned();

            tokio::spawn(async move {
                let _release_pool = acquire_pool.await;
                println!("key: {key} val: {val}");
                sleep(Duration::from_millis(1000)).await;
            })
        })
        .collect::<FuturesUnordered<_>>();

    let errgroup = futures::future::join_all(tasks).await;
    println!("{:?}", errgroup);

    Ok(())
}
