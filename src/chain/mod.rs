pub mod block;
pub mod blockchain;

#[cfg(test)]
mod tests {
    use super::block::Block;
    use super::blockchain::Blockchain;

    #[test]
    fn test_block_creation() {
        let block = Block::new(
            1,
            String::from("prev_hash"),
            vec![
                String::from("UserA checked in"),
                String::from("UserB got 1.2 Kred"),
                String::from("Validator elected: Node42"),
            ],
            42,
        );
        println!("{:#?}", block);
    }

    #[test]
    fn test_blockchain_growth() {
        let mut bc = Blockchain::new();
        bc.add_block(vec![String::from("UserA check-in")]);
        bc.add_block(vec![String::from("Validator selection event")]);

        assert_eq!(bc.chain.len(), 3);
        assert!(bc.is_valid_chain());
    }
}
