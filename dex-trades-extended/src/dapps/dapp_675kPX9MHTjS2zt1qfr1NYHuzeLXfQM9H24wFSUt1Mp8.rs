use substreams_solana::pb::sf::solana::r#type::v1::TokenBalance;

use crate::trade_instruction::TradeInstruction;

use crate::utils::get_mint;

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    post_token_balances: &Vec<TokenBalance>,
    accounts: &Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = u8::from(disc_bytes[0]);

    let mut result = None;

    match discriminator {
        9 => {
            result = Some(TradeInstruction {
                dapp_address: String::from("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"),
                name: String::from("SwapBaseIn"),
                amm: input_accounts.get(1).unwrap().to_string(),
                vault_a: get_vault_a(&input_accounts, post_token_balances, accounts),
                vault_b: get_vault_b(&input_accounts, post_token_balances, accounts),
                ..Default::default()
            });
        }
        11 => {
            result = Some(TradeInstruction {
                dapp_address: String::from("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"),
                name: String::from("SwapBaseOut"),
                amm: input_accounts.get(1).unwrap().to_string(),
                vault_a: get_vault_a(&input_accounts, post_token_balances, accounts),
                vault_b: get_vault_b(&input_accounts, post_token_balances, accounts),
                ..Default::default()
            });
        }
        _ => {}
    }

    return result;
}

fn get_vault_a(
    input_accounts: &Vec<String>,
    post_token_balances: &Vec<TokenBalance>,
    accounts: &Vec<String>,
) -> String {
    let mut vault_a = input_accounts.get(4).unwrap().to_string();
    let mint_a = get_mint(&vault_a, post_token_balances, accounts, "".to_string());

    if mint_a.is_empty() {
        vault_a = input_accounts.get(5).unwrap().to_string();
    }

    return vault_a;
}

fn get_vault_b(
    input_accounts: &Vec<String>,
    post_token_balances: &Vec<TokenBalance>,
    accounts: &Vec<String>,
) -> String {
    let mut vault_a_index = 4;

    let mut vault_a = input_accounts.get(4).unwrap().to_string();
    let mint_a = get_mint(&vault_a, post_token_balances, accounts, "".to_string());

    if mint_a.is_empty() {
        vault_a_index += 1;
        vault_a = input_accounts.get(vault_a_index).unwrap().to_string();
    }

    let mut vault_b_index = vault_a_index + 1;
    let mut vault_b = input_accounts.get(vault_b_index).unwrap().to_string();

    if vault_a == vault_b {
        vault_b_index += 1;
        vault_b = input_accounts.get(vault_b_index).unwrap().to_string();
    }

    return vault_b;
}
