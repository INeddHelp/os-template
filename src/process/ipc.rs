use crate::process::process::ProcessId;

// A message sent between processes via an IPC channel
pub struct Message {
    sender: ProcessId,
    data: Vec<u8>,
}

// An IPC channel for communication between two processes
pub struct IpcChannel {
    sender_id: ProcessId,
    receiver_id: ProcessId,
    messages: Vec<Message>,
}

impl IpcChannel {
    // Create a new IPC channel between the given sender and receiver processes
    pub fn new(sender_id: ProcessId, receiver_id: ProcessId) -> Self {
        Self {
            sender_id,
            receiver_id,
            messages: Vec::new(),
        }
    }

    // Send a message on the IPC channel
    pub fn send_message(&mut self, data: Vec<u8>) {
        let message = Message {
            sender: self.sender_id,
            data,
        };
        self.messages.push(message);
    }

    // Receive the next message from the IPC channel, if there is one
    pub fn receive_message(&mut self) -> Option<Message> {
        self.messages.pop()
    }
}

// TODO: Implement the DNS message encoding and decoding