

/// A server state where servers are placed on a map
#[derive(Debug, Clone)]
pub struct Server<'a> {
    /// The server name
    pub name: &'a str,
    /// The server location
    pub location: &'a str,
    /// The server coordinates
    pub coords: (f64, f64),
    /// The server status
    pub status: &'a str,
}
