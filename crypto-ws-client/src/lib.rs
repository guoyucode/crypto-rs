mod clients;

pub use clients::binance::*;

/// The public interface of every WebSocket client.
pub trait WSClient {
    /// Exchange specific WebSocket client.
    type Exchange;

    /// Create a new client.
    fn init(on_msg: fn(String)) -> Self::Exchange;

    /// Subscribe channels.
    fn subscribe(&mut self, channels: &[String]);

    /// Unsubscribe channels.
    fn unsubscribe(&mut self, channels: &[String]);

    /// Start the infinite loop until the server closes the connection.
    fn run(&mut self);

    /// Close the client.
    fn close(&mut self);
}