use std::collections::HashMap;

use super::secure_dns::DnsMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResolverId {
    DoH,
    DoT,
    DnsCrypt,
    Tor,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolverState {
    Disabled,
    Available,
    Active,
    Failed,
    Quarantined,
}

#[derive(Debug, Clone)]
pub struct ResolverEntry {
    pub id: ResolverId,
    pub mode: DnsMode,
    pub name: String,
    pub state: ResolverState,
    pub priority: u32,
}

impl ResolverEntry {
    pub fn new(
        id: ResolverId,
        mode: DnsMode,
        name: impl Into<String>,
        priority: u32,
    ) -> Self {
        Self {
            id,
            mode,
            name: name.into(),
            state: ResolverState::Available,
            priority,
        }
    }

    pub fn is_usable(&self) -> bool {
        matches!(
            self.state,
            ResolverState::Available | ResolverState::Active
        )
    }
}

#[derive(Debug, Default)]
pub struct ResolverRegistry {
    resolvers: HashMap<ResolverId, ResolverEntry>,
}

impl ResolverRegistry {
    pub fn new() -> Self {
        let mut registry = Self::default();

        registry.register(ResolverEntry::new(
            ResolverId::DoH,
            DnsMode::DoH,
            "DNS over HTTPS",
            10,
        ));

        registry.register(ResolverEntry::new(
            ResolverId::DoT,
            DnsMode::DoT,
            "DNS over TLS",
            20,
        ));

        registry.register(ResolverEntry::new(
            ResolverId::DnsCrypt,
            DnsMode::DnsCrypt,
            "DNSCrypt",
            30,
        ));

        registry.register(ResolverEntry::new(
            ResolverId::Tor,
            DnsMode::Tor,
            "Tor DNS",
            40,
        ));

        registry.register(ResolverEntry::new(
            ResolverId::System,
            DnsMode::System,
            "System DNS",
            100,
        ));

        registry
    }

    pub fn register(&mut self, resolver: ResolverEntry) {
        self.resolvers.insert(resolver.id, resolver);
    }

    pub fn remove(&mut self, id: ResolverId) {
        self.resolvers.remove(&id);
    }

    pub fn get(&self, id: ResolverId) -> Option<&ResolverEntry> {
        self.resolvers.get(&id)
    }

    pub fn get_mut(&mut self, id: ResolverId) -> Option<&mut ResolverEntry> {
        self.resolvers.get_mut(&id)
    }

    pub fn set_state(
        &mut self,
        id: ResolverId,
        state: ResolverState,
    ) -> bool {
        if let Some(resolver) = self.resolvers.get_mut(&id) {
            resolver.state = state;
            true
        } else {
            false
        }
    }

    pub fn best_available(&self) -> Option<&ResolverEntry> {
        self.resolvers
            .values()
            .filter(|resolver| resolver.is_usable())
            .min_by_key(|resolver| resolver.priority)
    }

    pub fn available(&self) -> Vec<&ResolverEntry> {
        let mut resolvers: Vec<_> = self
            .resolvers
            .values()
            .filter(|resolver| resolver.is_usable())
            .collect();

        resolvers.sort_by_key(|resolver| resolver.priority);
        resolvers
    }

    pub fn contains(&self, id: ResolverId) -> bool {
        self.resolvers.contains_key(&id)
    }

    pub fn all(&self) -> &HashMap<ResolverId, ResolverEntry> {
        &self.resolvers
    }
}
