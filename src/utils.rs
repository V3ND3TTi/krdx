use bs58;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

pub fn format_kred(koin: u64) -> String {
    let whole = koin / 100_000_000;
    let fraction = koin % 100_000_000;
    format!(
        "{}.{}",
        whole,
        format!("{:0>8}", fraction).trim_end_matches('0')
    )
}

pub fn btc_style_address(pubkey: &[u8]) -> String {
    let sha256 = Sha256::digest(pubkey);

    let mut ripemd = Ripemd160::new();
    ripemd.update(&sha256);
    let ripemd_hash = ripemd.finalize();

    let mut payload = vec![0x00]; // Bitcoin version byte
    payload.extend(&ripemd_hash);

    let checksum = Sha256::digest(&Sha256::digest(&payload));
    payload.extend(&checksum[..4]);

    let base58 = bs58::encode(payload).into_string();
    format!("KRDx{}", base58)
}
