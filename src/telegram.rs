use crate::config::Configuration;
use std::sync::Arc;
use teloxide::prelude::*;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct TelegramBot {
    telegram_token: String,
    telegram_user_id: i64,
    in_tx: Arc<Mutex<mpsc::Sender<String>>>,
    in_rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl TelegramBot {
    pub fn new(config: Configuration) -> TelegramBot {
        let (in_tx, in_rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(100);
        let in_rx_mutex = Arc::new(Mutex::new(in_rx));
        TelegramBot {
            telegram_token: config.telegram_token,
            telegram_user_id: config.telegram_user_id,
            in_tx: Arc::new(Mutex::new(in_tx)),
            in_rx: in_rx_mutex,
        }
    }

    pub async fn send_message(&self, message: String) {
        let mut tx = self.in_tx.lock().await;
        if let Err(e) = tx.send(message.clone()).await {
            println!("Failed to send message to channel: {:?}", e);
        }
    }

    pub async fn start(&self) -> JoinHandle<()> {
        println!("Starting telegram bot...");
        let bot = Bot::new(self.telegram_token.clone());
        let chat = ChatId(self.telegram_user_id);
        let in_rx_clone = self.in_rx.clone();

        let (tx, rx) = oneshot::channel();

        let handle = tokio::spawn(async move {
            if let Err(e) = tx.send(()) {
                println!("Failed to send startup confirmation: {:?}", e);
                return;
            }
            bot_main(bot, chat, in_rx_clone).await
        });

        if let Err(e) = rx.await {
            println!("Error waiting for bot startup: {:?}", e);
        }

        handle
    }
}

async fn bot_main(bot: Bot, chat: ChatId, in_rx: Arc<Mutex<mpsc::Receiver<String>>>) {
    loop {
        let mut rx = in_rx.lock().await;
        match rx.recv().await {
            Some(message) => {
                if let Err(e) = bot.send_message(chat, message).await {
                    println!("Failed to send message to Telegram: {:?}", e);
                }
            }
            None => {
                break;
            }
        }
    }
}
