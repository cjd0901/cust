use cust::{Item, Manager};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let (item_sender, item_receiver) = tokio::sync::mpsc::channel::<(u32, Arc<Mutex<Item>>)>(1);
    let mut manager = Manager {
        item_receiver: Box::new(item_receiver),
        items: Arc::new(Mutex::new(HashMap::new())),
    };

    let item = Arc::new(Mutex::new(Item { id: 1 }));
    if let Err(_) = item_sender.send((1, Arc::clone(&item))).await {
        println!("error");
    };

    tokio::spawn(async move {
        loop {
            tokio::select! {
                item = manager.item_receiver.recv() => {
                    match item {
                        Some((id, item)) => {
                            let mut items = manager.items.lock().unwrap();
                            items.entry(id).or_insert(Arc::clone(&item));
                            let mut item = item.lock().unwrap();
                            item.id = 2;
                        },
                        None => {
                            println!("None");
                        }
                    }
                }
            }
        }
    });

    sleep(Duration::from_secs(1)).await;
    println!("{}", item.lock().unwrap().id);

    thread::park();
}
