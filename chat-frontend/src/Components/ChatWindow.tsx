import React, { useState, useEffect } from 'react';
import axios from 'axios';
import Message from './Message';
import config from '../config';

interface Message {
    id: number;
    body: string;
}

const ChatWindow: React.FC = () => {
    const [messages, setMessages] = useState<Message[]>([]);
    const [newMessage, setNewMessage] = useState<string>('');
    const [editingMessage, setEditingMessage] = useState<Message | null>(null);

    useEffect(() => {
        // Fetch initial messages from API
        axios.get(`${config.apiBaseUrl}/api/messages`)
            .then(response => {
                setMessages(response.data);
            })
            .catch(error => {
                console.error('Error fetching messages:', error);
            });
    }, []);

    const handleSendMessage = () => {
        if (newMessage.trim() === '') return;

        if (editingMessage) {
            axios.put(`${config.apiBaseUrl}/api/messages/${editingMessage.id}`, { body: newMessage })
                .then(response => {
                    setMessages(messages.map(msg => msg.id === editingMessage.id ? response.data : msg));
                    setNewMessage('');
                    setEditingMessage(null);
                })
                .catch(error => {
                    console.error('Error updating message:', error);
                });
        } else {
            const message = { body: newMessage };

            axios.post(`${config.apiBaseUrl}/api/messages`, message)
                .then(response => {
                    setMessages([...messages, response.data]);
                    setNewMessage('');
                })
                .catch(error => {
                    console.error('Error sending message:', error);
                });
        }
    };

    const handleEditMessage = (message: Message) => {
        setEditingMessage(message);
        setNewMessage(message.body);
    };

    return (
        <div>
            <div className="message-list">
                {messages.map(msg => (
                    <Message key={msg.id} message={msg} onEdit={() => handleEditMessage(msg)} />
                ))}
            </div>
            <div className="message-input">
                <input
                    type="text"
                    value={newMessage}
                    onChange={(e) => setNewMessage(e.target.value)}
                    placeholder="Type a message"
                />
                <button onClick={handleSendMessage}>
                    {editingMessage ? 'Update' : 'Send'}
                </button>
            </div>
        </div>
    );
};

export default ChatWindow;
