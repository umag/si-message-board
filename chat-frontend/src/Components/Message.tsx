import React from 'react';

interface MessageProps {
    message: {
        id: number;
        body: string;
    };
    onEdit: () => void;
    onDelete: () => void;
}

const Message: React.FC<MessageProps> = ({ message, onEdit, onDelete }) => {
    return (
        <div className="message">
            {message.body}
            <button onClick={onEdit} style={{ marginLeft: '10px' }}>Edit</button>
            <button onClick={onDelete} style={{ marginLeft: '10px', backgroundColor: 'red', color: 'white' }}>Delete</button>
        </div>
    );
};

export default Message;
