pub mod network;
pub mod client;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::client::connect();
    }
}