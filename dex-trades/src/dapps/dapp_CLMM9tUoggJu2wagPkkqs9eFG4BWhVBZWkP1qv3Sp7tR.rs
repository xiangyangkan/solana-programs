use crate::trade_instruction::TradeInstruction;

const SWAP_WITH_PARTNER_DISCRIMINATOR: u64 =
    u64::from_le_bytes([133, 215, 191, 214, 102, 243, 55, 25]);

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        SWAP_WITH_PARTNER_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("CLMM9tUoggJu2wagPkkqs9eFG4BWhVBZWkP1qv3Sp7tR"),
                name: String::from("SwapWithPartner"),
                amm: accounts.get(1).unwrap().to_string(),
                vault_a: accounts.get(6).unwrap().to_string(),
                vault_b: accounts.get(7).unwrap().to_string(),
                ..Default::default()
            });
        }
        _ => {}
    }

    return result;
}
