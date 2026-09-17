use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RouteTarget {
    Direct,
    Proxy,
    Vpn,
    Tor,
    I2p,
    Nym,
    Lokinet,
    Yggdrasil,
    Ipfs,
    Freenet,
    GnUnet,
    Hyphanet,
    Blocked,
}

#[derive(Debug, Clone)]
pub struct RouteRule {
    pub context: String,
    pub target: RouteTarget,
}

#[derive(Debug)]
pub struct RoutingManager {
    default_route: RouteTarget,
    routes: HashMap<String, RouteTarget>,
}

impl RoutingManager {
    pub fn new() -> Self {
        Self {
            default_route: RouteTarget::Direct,
            routes: HashMap::new(),
        }
    }

    pub fn set_default_route(&mut self, target: RouteTarget) {
        self.default_route = target;
    }

    pub fn default_route(&self) -> RouteTarget {
        self.default_route
    }

    pub fn set_route(
        &mut self,
        context: impl Into<String>,
        target: RouteTarget,
    ) {
        self.routes.insert(context.into(), target);
    }

    pub fn remove_route(&mut self, context: &str) -> bool {
        self.routes.remove(context).is_some()
    }

    pub fn route_for(&self, context: &str) -> RouteTarget {
        self.routes
            .get(context)
            .copied()
            .unwrap_or(self.default_route)
    }

    pub fn is_blocked(&self, context: &str) -> bool {
        self.route_for(context) == RouteTarget::Blocked
    }

    pub fn routes(&self) -> impl Iterator<Item = (&String, &RouteTarget)> {
        self.routes.iter()
    }

    pub fn clear(&mut self) {
        self.routes.clear();
    }
}

impl Default for RoutingManager {
    fn default() -> Self {
        Self::new()
    }
}
