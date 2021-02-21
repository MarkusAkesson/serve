use crate::cli;
use crate::DEFAULT_PORT;

use simplelog::LevelFilter;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct Config {
    ip: Ipv4Addr,
    port: u16,
    verbosity: LevelFilter,
}

impl Config {
    pub fn from_args() -> Self {
        let matches = cli::args();

        let ip = match matches.value_of("ip") {
            Some(ip) => ip.parse().unwrap_or(Ipv4Addr::LOCALHOST),
            None => Ipv4Addr::LOCALHOST,
        };

        let port = matches
            .value_of("port")
            .and_then(|port| port.parse().ok())
            .unwrap_or(DEFAULT_PORT);

        let verbosity = match matches.occurrences_of("v") {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        Config {
            ip,
            port,
            verbosity,
        }
    }

    pub fn ip(&self) -> Ipv4Addr {
        self.ip
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(self.ip), self.port)
    }

    pub fn log_level(&self) -> LevelFilter {
        self.verbosity
    }
}
