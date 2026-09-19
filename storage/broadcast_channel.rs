use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct BroadcastMessage {
    pub channel: String,
    pub sender_id: u64,
    pub payload: Vec<u8>,
}

impl BroadcastMessage {
    pub fn new(
        channel: impl Into<String>,
        sender_id: u64,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            channel: channel.into(),
            sender_id,
            payload,
        }
    }
}

#[derive(Debug)]
pub struct BroadcastChannel {
    pub id: u64,
    pub site: String,
    pub name: String,
    messages: VecDeque<BroadcastMessage>,
}

impl BroadcastChannel {
    pub fn new(
        id: u64,
        site: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            site: site.into(),
            name: name.into(),
            messages: VecDeque::new(),
        }
    }

    pub fn publish(
        &mut self,
        sender_id: u64,
        payload: Vec<u8>,
    ) {
        self.messages.push_back(
            BroadcastMessage::new(
                self.name.clone(),
                sender_id,
                payload,
            ),
        );
    }

    pub fn receive(&mut self) -> Option<BroadcastMessage> {
        self.messages.pop_front()
    }

    pub fn pending_messages(&self) -> usize {
        self.messages.len()
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

#[derive(Debug, Default)]
pub struct BroadcastChannelManager {
    channels: HashMap<(String, String), Vec<BroadcastChannel>>,
    next_id: u64,
}

impl BroadcastChannelManager {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create(
        &mut self,
        site: &str,
        name: &str,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let channel = BroadcastChannel::new(
            id,
            site,
            name,
        );

        self.channels
            .entry((site.to_string(), name.to_string()))
            .or_default()
            .push(channel);

        id
    }

    pub fn post_message(
        &mut self,
        site: &str,
        name: &str,
        sender_id: u64,
        payload: Vec<u8>,
    ) {
        if let Some(channels) = self
            .channels
            .get_mut(&(site.to_string(), name.to_string()))
        {
            for channel in channels {
                if channel.id != sender_id {
                    channel.publish(
                        sender_id,
                        payload.clone(),
                    );
                }
            }
        }
    }

    pub fn receive(
        &mut self,
        site: &str,
        name: &str,
        channel_id: u64,
    ) -> Option<BroadcastMessage> {
        self.channels
            .get_mut(&(site.to_string(), name.to_string()))
            .and_then(|channels| {
                channels
                    .iter_mut()
                    .find(|channel| channel.id == channel_id)
            })
            .and_then(BroadcastChannel::receive)
    }

    pub fn close(
        &mut self,
        site: &str,
        name: &str,
        channel_id: u64,
    ) -> bool {
        let key = (site.to_string(), name.to_string());

        let Some(channels) = self.channels.get_mut(&key) else {
            return false;
        };

        let original_len = channels.len();

        channels.retain(|channel| channel.id != channel_id);

        if channels.is_empty() {
            self.channels.remove(&key);
        }

        original_len != channels.len()
    }

    pub fn clear_site(&mut self, site: &str) {
        self.channels
            .retain(|(channel_site, _), _| channel_site != site);
    }

    pub fn clear_all(&mut self) {
        self.channels.clear();
    }
}
