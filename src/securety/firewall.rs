// TODO: Implement Firewall functionality

pub struct Firewall {
    rules: Vec<Rule>,
}

pub struct Rule {
    protocol: String,
    source_ip: String,
    destination_ip: String,
    action: Action,
}

pub enum Action {
    Allow,
    Block,
}

impl Firewall {
    pub fn new() -> Firewall {
        Firewall { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, rule: &Rule) -> bool {
        if let Some(index) = self.rules.iter().position(|r| r == rule) {
            self.rules.remove(index);
            return true;
        }

        false
    }

    pub fn check_packet(&self, packet: &Packet) -> Action {
        // TODO: Implement check_packet functionality

        Action::Allow
    }
}

pub struct Packet {
    protocol: String,
    source_ip: String,
    destination_ip: String,
    payload: Vec<u8>,
}

impl Packet {
    pub fn new(protocol: String, source_ip: String, destination_ip: String, payload: Vec<u8>) -> Packet {
        Packet {
            protocol,
            source_ip,
            destination_ip,
            payload,
        }
    }
}
