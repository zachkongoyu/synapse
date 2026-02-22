struct Message {
    role: String,
    content: Option<String>,
}

pub struct Memory {
    messages: Vec<Message>,
}

impl Memory {
    pub fn read(&self, last: Option<usize>) -> &[Message] {
        match last {
            None => &self.messages,
            Some(n) => {
                let start = self.messages.len().saturating_sub(n);
                &self.messages[start..]
            }
        }
    }

    pub fn extract(&self) -> Vec<Message> { self.messages.clone() }

    pub fn memorize(&mut self, msg: Message) {
        // For simplicity, we just push the message to the end of the vector. 
        // But I want memorize to do more incredible things.
        self.messages.push(msg); 
    }
}