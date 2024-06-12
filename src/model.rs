

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone,Debug, Serialize)]
pub struct Message {
    pub id: u64,
    pub body: String,
}
#[derive(Debug, Deserialize)]
pub struct MessageForCreate {
    pub body: String,
}

#[derive(Clone)]
pub struct ModelController {
    messages_store: Arc<Mutex<Vec<Option<Message>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            messages_store: Arc::default(),
        })
    }

    pub async fn create_message(&self, message_fc: MessageForCreate) -> Result<Message> {
    let mut store = self.messages_store.lock().unwrap();
    let id = store.len() as u64;
    let message = Message {
        id,
        body: message_fc.body,
    };
    store.push(Some(Message.clone()));

    Ok(Message)
    }

    pub async fn list_messages(&self) -> Result<Vec<Message>> {
        let store = self.messages_store.lock().unwrap();
        let messages = store.iter().filter_map(|t| t.clone()).collect();
        Ok(messages)
    }

    pub async fn delete_message(&self, id: u64) -> Result<Message> {
        let mut store = self.messages_store.lock().unwrap();
        let Message = 
        store.get_mut(id as usize).and_then(|t| t.take());
        message.ok_or(Error::MessageDeleteFailIdNotFound { id })
    }

    pub async fn update_message(&self, id: u64, message_fc: MessageForCreate) -> Result<Message> {
        let mut store = self.messages_store.lock().unwrap();
        let message = store.get_mut(id as usize).ok_or(Error::MessageUpdateFailIdNotFound { id })?;
        message.body = message_fc.body.clone();
        Ok(message)
    }
}