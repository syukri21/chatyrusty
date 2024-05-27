use tokio::sync::broadcast::{self, Receiver, Sender};

#[derive(Clone)]
pub struct EmailVerifiedChannelImpl {
    tx: Sender<EmailVerifiedMessage>,
}

#[derive(Clone, Debug)]
pub struct EmailVerifiedMessage {
    pub user_id: String,
    pub message: String,
}

pub trait EmailVerifiedChannel {
    fn sender(&self) -> Sender<EmailVerifiedMessage>;
    fn receiver(&self) -> Receiver<EmailVerifiedMessage>;
    fn send(
        &self,
        msg: EmailVerifiedMessage,
    ) -> Result<usize, broadcast::error::SendError<EmailVerifiedMessage>>;
}

impl EmailVerifiedChannelImpl {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel::<EmailVerifiedMessage>(100);
        Self { tx }
    }
}

impl EmailVerifiedChannel for EmailVerifiedChannelImpl {
    fn sender(&self) -> Sender<EmailVerifiedMessage> {
        self.tx.clone()
    }

    fn receiver(&self) -> Receiver<EmailVerifiedMessage> {
        self.tx.subscribe()
    }

    fn send(
        &self,
        msg: EmailVerifiedMessage,
    ) -> Result<usize, broadcast::error::SendError<EmailVerifiedMessage>> {
        self.tx.send(msg)
    }
}
