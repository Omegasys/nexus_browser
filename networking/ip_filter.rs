use std::collections::HashSet;
use std::net::IpAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IpVersion {
    V4,
    V6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IpFilterAction {
    Allow,
    Block,
    Log,
}

#[derive(Debug, Clone)]
pub struct IpRule {
    pub address: IpAddr,
    pub prefix_length: u8,
    pub action: IpFilterAction,
}

impl IpRule {
    pub fn new(address: IpAddr, prefix_length: u8, action: IpFilterAction) -> Self {
        Self {
            address,
            prefix_length,
            action,
        }
    }

    pub fn matches(&self, target: IpAddr) -> bool {
        if self.address.is_ipv4() != target.is_ipv4() {
            return false;
        }

        let network_bits = match self.address {
            IpAddr::V4(_) => 32,
            IpAddr::V6(_) => 128,
        };

        if self.prefix_length > network_bits {
            return false;
        }

        let address_value = match self.address {
            IpAddr::V4(value) => u128::from(u32::from(value)),
            IpAddr::V6(value) => u128::from(value),
        };

        let target_value = match target {
            IpAddr::V4(value) => u128::from(u32::from(value)),
            IpAddr::V6(value) => u128::from(value),
        };

        let mask = if self.prefix_length == 0 {
            0
        } else {
            u128::MAX << (128 - self.prefix_length as u32)
        };

        let adjusted_mask = match self.address {
            IpAddr::V4(_) => {
                if self.prefix_length == 0 {
                    0
                } else {
                    u128::from(u32::MAX << (32 - self.prefix_length as u32))
                }
            }
            IpAddr::V6(_) => mask,
        };

        (address_value & adjusted_mask) == (target_value & adjusted_mask)
    }
}

#[derive(Debug, Default)]
pub struct IpFilter {
    rules: Vec<IpRule>,
    blocked_addresses: HashSet<IpAddr>,
}

impl IpFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, rule: IpRule) {
        self.rules.push(rule);
    }

    pub fn block(&mut self, address: IpAddr) {
        self.blocked_addresses.insert(address);
    }

    pub fn unblock(&mut self, address: &IpAddr) {
        self.blocked_addresses.remove(address);
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    pub fn evaluate(&self, address: IpAddr) -> IpFilterAction {
        if self.blocked_addresses.contains(&address) {
            return IpFilterAction::Block;
        }

        for rule in self.rules.iter().rev() {
            if rule.matches(address) {
                return rule.action;
            }
        }

        IpFilterAction::Allow
    }

    pub fn allows(&self, address: IpAddr) -> bool {
        self.evaluate(address) == IpFilterAction::Allow
    }

    pub fn rules(&self) -> &[IpRule] {
        &self.rules
    }
}
