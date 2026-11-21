use std::net::IpAddr;

pub(crate)  struct InstanceInfo {
    pub(crate) vmid: u64,
    pub(crate) name: String,
    pub(crate) ip: IpAddr,
}

pub(crate) enum InstanceUptime {
    Stopped,
    Running,
}

pub(crate) struct InstanceResourceUsage {
    pub(crate) vmid: u64,
    pub(crate) status: InstanceUptime,
    pub(crate) uptime: u128,

    pub(crate) cpu: f64,
    pub(crate) max_cpu: f64,

    pub(crate) mem: f64,
    pub(crate) max_mem: f64,
}

pub(crate) enum RouteType {
    TCP,
    UDP,
    BOTH
}

pub(crate) struct Routing {
    pub(crate) vmid: u64,
    pub(crate) route_type: RouteType,
    pub(crate) internal_port: u16,
    pub(crate) external_port: u16,
}

pub(crate) struct Forwarding {
    pub(crate) vmid: u64,
    pub(crate) internal_port: u16,
    pub(crate) target_host: String, // It will contain the entire host string e.g host1.example.com 
}