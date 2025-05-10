pub mod block;
pub mod blockchain;
pub mod ledger;

#[cfg(test)]
mod tests {
    use super::blockchain::{Blockchain, CoinbaseDistribution};
    use crate::utils::format_kred;

    #[test]
    fn test_blockchain_growth_and_rewards() {
        let founder_addresses = vec![String::from("KRDxFounder1"), String::from("KRDxFounder2")];

        let faucet_address = String::from("KRDxFaucet");

        let dist = CoinbaseDistribution {
            faucet_address: faucet_address.clone(),
            founder_addresses: founder_addresses.clone(),
        };

        let mut blockchain = Blockchain::new(dist);

        blockchain.add_block(
            vec![String::from("First mined block")],
            founder_addresses[0].clone(),
        );

        blockchain.add_block(
            vec![String::from("Second mined block")],
            founder_addresses[1].clone(),
        );

        assert_eq!(blockchain.chain.len(), 3);

        let ledger = &blockchain.ledger;
        let f_balance = ledger.get_balance(&faucet_address);
        let f1_balance = ledger.get_balance(&founder_addresses[0]);
        let f2_balance = ledger.get_balance(&founder_addresses[1]);

        println!("Faucet: {} KRD", format_kred(f_balance));
        println!("Founder 1: {} KRDX", format_kred(f1_balance));
        println!("Founder 2: {} KRED", format_kred(f2_balance));

        assert!(f_balance > 0);
        assert!(f1_balance > 0);
        assert!(f2_balance > 0);
    }
}
