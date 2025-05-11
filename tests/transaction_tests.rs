use krdx::wallet::Wallet;

#[test]
fn test_transaction_create_and_verify() {
    let sender = Wallet::new();
    let recipient = Wallet::new();

    let tx = sender.create_transaction(&recipient.address, 42_000_000);
    assert!(tx.is_valid(), "Transaction shold be valid");
}
