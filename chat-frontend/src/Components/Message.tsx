import React from 'react';

interface MessageProps {
    message: {
        id: number;
        body: string;
    };
    onEdit: () => void;
}

const Message: React.FC<MessageProps> = ({ message, onEdit }) => {
    return (
        <div className="message">
            {message.body}
            <button onClick={onEdit} style={{ marginLeft: '10px' }}>Edit</button>
        </div>
    );
};

export default Message;
