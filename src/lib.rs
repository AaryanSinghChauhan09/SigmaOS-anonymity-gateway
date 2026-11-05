#[derive(Debug, Clone)]
pub struct VirtualInterface {
    pub name: String,
    pub ip_subnet: String,
    pub is_isolated: bool,
}

#[derive(Debug, Default)]
pub struct AnonymityGateway {
    interfaces: Vec<VirtualInterface>,
    tor_transport_port: u16,
}

impl AnonymityGateway {
    pub fn new(port: u16) -> Self {
        Self { interfaces: Vec::new(), tor_transport_port: port }
    }
    pub fn add_interface(&mut self, name: &str, subnet: &str, isolated: bool) {
        self.interfaces.push(VirtualInterface {
            name: name.to_string(),
            ip_subnet: subnet.to_string(),
            is_isolated: isolated,
        });
    }
    pub fn is_workstation_isolated(&self, name: &str) -> bool {
        self.interfaces.iter().find(|i| i.name == name).map(|i| i.is_isolated).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_anonymity_gateway_isolation() {
        let mut gw = AnonymityGateway::new(9040);
        gw.add_interface("veth-ws", "10.152.152.0/24", true);
        assert!(gw.is_workstation_isolated("veth-ws"));
        assert!(!gw.is_workstation_isolated("veth-ext"));
    }
}
