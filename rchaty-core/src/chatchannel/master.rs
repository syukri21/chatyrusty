use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};

use tokio::sync::broadcast::{self, Sender};

#[derive(Clone)]
pub struct MasterChannelImpl {
    tx: Arc<Mutex<HashMap<String, Sender<InnerNodeChannelData>>>>,
}

// type RootChannelTx = Arc<Mutex<HashMap<String, Sender<Box<dyn ChannelData>>>>>;
type InnerNodeChannelData = Arc<dyn ChannelData + Send + Sync>;

impl MasterChannelImpl {
    pub fn new() -> Self {
        let channels = Arc::new(Mutex::new(HashMap::new()));
        Self { tx: channels }
    }
}

#[async_trait::async_trait]
pub trait MasterChannel {
    fn create_channel(&self, user_id: &str);
    async fn tx(&self, user_id: &str) -> Option<Sender<Arc<dyn ChannelData + Send + Sync>>>;
}

#[async_trait::async_trait]
impl MasterChannel for MasterChannelImpl {
    fn create_channel(&self, user_id: &str) {
        let mut tx = self.tx.lock().unwrap();
        match tx.get(user_id) {
            Some(_) => {
                tracing::info!("channel already exists for user_id: {}", user_id);
            }
            None => {
                let (node_tx, _) = broadcast::channel::<InnerNodeChannelData>(100);
                tx.insert(user_id.to_string(), node_tx);
            }
        }
    }

    async fn tx(&self, user_id: &str) -> Option<Sender<Arc<dyn ChannelData + Send + Sync>>> {
        let tx = self.tx.lock().unwrap();
        match tx.get(user_id) {
            Some(tx) => Some(tx.clone()),
            None => {
                tracing::info!("channel does not exist for user_id: {}", user_id);
                None
            }
        }
    }
}

pub trait ChannelData: Send + Sync + Debug {
    fn data(&self) -> String;
}

#[derive(Debug)]
pub struct ChannelDataImpl {
    data: String,
}

impl ChannelDataImpl {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

impl ChannelData for ChannelDataImpl {
    fn data(&self) -> String {
        self.data.clone()
    }
}
