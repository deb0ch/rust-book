
pub mod client;
pub mod network;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        super::client::connect()
    }
}