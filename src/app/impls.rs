use super::*;

impl Default for App {
    fn default() -> Self {
        Self {
            address: SocketAddr::from(([127, 0, 0, 1], 8080)),
            ..Default::default()
        }
    }
}