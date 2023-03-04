use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Receiver;

pub struct Item {
    pub id: u32,
}

pub struct Manager {
    pub items: Arc<Mutex<HashMap<u32, Arc<Mutex<Item>>>>>,
    pub item_receiver: Box<Receiver<(u32, Arc<Mutex<Item>>)>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
