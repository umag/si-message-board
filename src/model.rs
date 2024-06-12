
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, sync::{Arc, Mutex}};

#[derive(Clone,Debug, Serialize)]
pub struct Message {
    pub id: u64,
    pub body: String,
}

// New type for creating a message, no id only body
#[derive(Debug, Deserialize)]
pub struct MessageForCreate {
    pub body: String,
}

// Rust type magick to have atomic reference counting and mutex for Vector of 
// optional messages
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
// Create a message and return it
    pub async fn create_message(&self, message_fc: MessageForCreate) -> Result<Message> {
    let mut store = self.messages_store.lock().unwrap();
    let id = store.len() as u64;
    let message = Message {
        id,
        body: message_fc.body,
    };
    store.push(Some(message.clone()));

    Ok(message)
    }
// List all messages
    pub async fn list_messages(&self) -> Result<Vec<Message>> {
        let store = self.messages_store.lock().unwrap();
        let messages = store.iter().filter_map(|m| m.clone()).collect();
        Ok(messages)
    }
// Delete a message by id and return it
    pub async fn delete_message(&self, id: u64) -> Result<Message> {
        let mut store = self.messages_store.lock().unwrap();
        let message = 
        store.get_mut(id as usize).and_then(|m| m.take());
        message.ok_or(Error::MessageDeleteFailIdNotFound { id })
    }
// Update a message by id and return old message
    pub async fn update_message(&self, id: u64, message_fc: MessageForCreate) -> Result<Message> {
        let mut store = self.messages_store.lock().unwrap();
        let mut new_message = Message {
            id,
            body: message_fc.body.clone(),
        };
        let message = 
        store.get_mut(id as usize).and_then(|m| m.replace(new_message.clone()));
        message.ok_or(Error::MessageUpdateFailIdNotFound { id })
    }
}