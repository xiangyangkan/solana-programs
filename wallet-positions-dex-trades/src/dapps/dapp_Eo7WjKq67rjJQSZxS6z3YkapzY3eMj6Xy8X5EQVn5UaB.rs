const SWAP_WITH_PARTNER_DISCRIMINATOR: u64 =
    u64::from_le_bytes([248, 198, 158, 145, 225, 117, 135, 200]);

pub fn is_trade_instruction(bytes_stream: Vec<u8>) -> bool {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    match discriminator {
        SWAP_WITH_PARTNER_DISCRIMINATOR => {
            return true;
        }
        _ => {}
    }

    false
}
