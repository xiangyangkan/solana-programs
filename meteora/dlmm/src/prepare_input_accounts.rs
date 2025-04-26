// This file will contain account preparation logic for Meteora DLMM
// It should be implemented based on Meteora's DLMM program structure

use std::collections::HashMap;
use crate::instructions::InstructionType;

// Struct to hold input accounts mapping results
pub struct InputAccounts {
    pub accounts: Vec<String>,
    pub input_accounts: HashMap<String, String>,
}

// Maps account indexes to their corresponding roles based on the instruction type
pub fn map_accounts(
    account_indices: &[u8],
    instruction_type: InstructionType,
) -> InputAccounts {
    let mut role_by_index = HashMap::new();
    
    // Create a mapping of indices to account roles based on instruction type
    match instruction_type {
        InstructionType::Swap | InstructionType::SwapWithPriceImpact | InstructionType::SwapExactOut => {
            // Match V1 IDL definition (15 accounts)
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 2, "reserveX");
            assign_if_exists(&mut role_by_index, 3, "reserveY");
            assign_if_exists(&mut role_by_index, 4, "userTokenIn");
            assign_if_exists(&mut role_by_index, 5, "userTokenOut");
            assign_if_exists(&mut role_by_index, 6, "tokenXMint");
            assign_if_exists(&mut role_by_index, 7, "tokenYMint");
            assign_if_exists(&mut role_by_index, 8, "oracle");
            assign_if_exists(&mut role_by_index, 9, "hostFeeIn");
            assign_if_exists(&mut role_by_index, 10, "user");
            assign_if_exists(&mut role_by_index, 11, "tokenXProgram");
            assign_if_exists(&mut role_by_index, 12, "tokenYProgram");
            assign_if_exists(&mut role_by_index, 13, "eventAuthority");
            assign_if_exists(&mut role_by_index, 14, "program");
        },
        
        // Core pool operations
        InstructionType::InitializeLbPair => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 2, "tokenMintX");
            assign_if_exists(&mut role_by_index, 3, "tokenMintY");
            assign_if_exists(&mut role_by_index, 4, "reserveX");
            assign_if_exists(&mut role_by_index, 5, "reserveY");
            assign_if_exists(&mut role_by_index, 6, "oracle");
            assign_if_exists(&mut role_by_index, 7, "presetParameter");
            assign_if_exists(&mut role_by_index, 8, "funder");
            assign_if_exists(&mut role_by_index, 9, "tokenProgram");
            assign_if_exists(&mut role_by_index, 10, "systemProgram");
            assign_if_exists(&mut role_by_index, 11, "rent");
            assign_if_exists(&mut role_by_index, 12, "eventAuthority");
            assign_if_exists(&mut role_by_index, 13, "program");
        },
        
        InstructionType::InitializePermissionLbPair => {
            assign_if_exists(&mut role_by_index, 0, "base");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 3, "tokenMintX");
            assign_if_exists(&mut role_by_index, 4, "tokenMintY");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "oracle");
            assign_if_exists(&mut role_by_index, 8, "presetParameter");
            assign_if_exists(&mut role_by_index, 9, "admin");
            assign_if_exists(&mut role_by_index, 10, "tokenProgram");
            assign_if_exists(&mut role_by_index, 11, "systemProgram");
            assign_if_exists(&mut role_by_index, 12, "rent");
            assign_if_exists(&mut role_by_index, 13, "eventAuthority");
            assign_if_exists(&mut role_by_index, 14, "program");
        },
        
        // Liquidity operations
        InstructionType::AddLiquidity | 
        InstructionType::AddLiquidityByWeight | 
        InstructionType::AddLiquidityByStrategy => {
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 3, "userTokenX");
            assign_if_exists(&mut role_by_index, 4, "userTokenY");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "tokenXMint");
            assign_if_exists(&mut role_by_index, 8, "tokenYMint");
            assign_if_exists(&mut role_by_index, 9, "binArrayLower");
            assign_if_exists(&mut role_by_index, 10, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 11, "sender");
            assign_if_exists(&mut role_by_index, 12, "tokenXProgram");
            assign_if_exists(&mut role_by_index, 13, "tokenYProgram");
            assign_if_exists(&mut role_by_index, 14, "eventAuthority");
            assign_if_exists(&mut role_by_index, 15, "program");
        },
        
        InstructionType::AddLiquidityOneSide | 
        InstructionType::AddLiquidityByStrategyOneSide | 
        InstructionType::AddLiquidityOneSidePrecise => {
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 3, "userToken");
            assign_if_exists(&mut role_by_index, 4, "reserve");
            assign_if_exists(&mut role_by_index, 5, "tokenMint");
            assign_if_exists(&mut role_by_index, 6, "binArrayLower");
            assign_if_exists(&mut role_by_index, 7, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 8, "sender");
            assign_if_exists(&mut role_by_index, 9, "tokenProgram");
            assign_if_exists(&mut role_by_index, 10, "eventAuthority");
            assign_if_exists(&mut role_by_index, 11, "program");
        },
        
        InstructionType::RemoveLiquidity | 
        InstructionType::RemoveAllLiquidity | 
        InstructionType::RemoveLiquidityByRange => {
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 3, "userTokenX");
            assign_if_exists(&mut role_by_index, 4, "userTokenY");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "tokenXMint");
            assign_if_exists(&mut role_by_index, 8, "tokenYMint");
            assign_if_exists(&mut role_by_index, 9, "binArrayLower");
            assign_if_exists(&mut role_by_index, 10, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 11, "sender");
            assign_if_exists(&mut role_by_index, 12, "tokenXProgram");
            assign_if_exists(&mut role_by_index, 13, "tokenYProgram");
            assign_if_exists(&mut role_by_index, 14, "eventAuthority");
            assign_if_exists(&mut role_by_index, 15, "program");
        },
        
        // Position management
        InstructionType::InitializePosition | 
        InstructionType::InitializePositionPda => {
            assign_if_exists(&mut role_by_index, 0, "payer");
            assign_if_exists(&mut role_by_index, 1, "position");
            assign_if_exists(&mut role_by_index, 2, "lbPair");
            assign_if_exists(&mut role_by_index, 3, "owner");
            assign_if_exists(&mut role_by_index, 4, "systemProgram");
            assign_if_exists(&mut role_by_index, 5, "rent");
            assign_if_exists(&mut role_by_index, 6, "eventAuthority");
            assign_if_exists(&mut role_by_index, 7, "program");
        },
        
        InstructionType::ClosePosition => {
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayLower");
            assign_if_exists(&mut role_by_index, 3, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 4, "sender");
            assign_if_exists(&mut role_by_index, 5, "rentReceiver");
            assign_if_exists(&mut role_by_index, 6, "eventAuthority");
            assign_if_exists(&mut role_by_index, 7, "program");
        },
        
        // Fee operations
        InstructionType::ClaimFee => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "position");
            assign_if_exists(&mut role_by_index, 2, "binArrayLower");
            assign_if_exists(&mut role_by_index, 3, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 4, "sender");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "userTokenX");
            assign_if_exists(&mut role_by_index, 8, "userTokenY");
            assign_if_exists(&mut role_by_index, 9, "tokenXMint");
            assign_if_exists(&mut role_by_index, 10, "tokenYMint");
            assign_if_exists(&mut role_by_index, 11, "tokenProgram");
            assign_if_exists(&mut role_by_index, 12, "eventAuthority");
            assign_if_exists(&mut role_by_index, 13, "program");
        },
        
        // Reward operations
        InstructionType::ClaimReward => {
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "position");
            assign_if_exists(&mut role_by_index, 2, "binArrayLower");
            assign_if_exists(&mut role_by_index, 3, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 4, "owner");
            assign_if_exists(&mut role_by_index, 5, "rewardVault");
            assign_if_exists(&mut role_by_index, 6, "rewardMint");
            assign_if_exists(&mut role_by_index, 7, "userTokenAccount");
            assign_if_exists(&mut role_by_index, 8, "tokenProgram");
            assign_if_exists(&mut role_by_index, 9, "eventAuthority");
            assign_if_exists(&mut role_by_index, 10, "program");
        },
        
        // V2 Swap variants share similar base accounts
        InstructionType::Swap2 | InstructionType::SwapExactOut2 | InstructionType::SwapWithPriceImpact2 => {
             // Match V2 IDL definition (16 base accounts)
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArrayBitmapExtension"); // Added based on IDL
            assign_if_exists(&mut role_by_index, 2, "reserveX");  // Index adjusted
            assign_if_exists(&mut role_by_index, 3, "reserveY");  // Index adjusted
            assign_if_exists(&mut role_by_index, 4, "userTokenIn"); // Index adjusted
            assign_if_exists(&mut role_by_index, 5, "userTokenOut"); // Index adjusted
            assign_if_exists(&mut role_by_index, 6, "tokenXMint"); // Index adjusted
            assign_if_exists(&mut role_by_index, 7, "tokenYMint"); // Index adjusted
            assign_if_exists(&mut role_by_index, 8, "oracle"); // Index adjusted
            assign_if_exists(&mut role_by_index, 9, "hostFeeIn"); // Index adjusted
            assign_if_exists(&mut role_by_index, 10, "user"); // Index adjusted, renamed from owner
            assign_if_exists(&mut role_by_index, 11, "tokenXProgram"); // Index adjusted
            assign_if_exists(&mut role_by_index, 12, "tokenYProgram"); // Added based on IDL
            assign_if_exists(&mut role_by_index, 13, "memoProgram"); // Added based on IDL
            assign_if_exists(&mut role_by_index, 14, "eventAuthority"); // Index adjusted
            assign_if_exists(&mut role_by_index, 15, "program"); // Index adjusted
            // Additional accounts beyond index 15 are handled dynamically via RemainingAccountsInfo argument
        },
        
        // Add Mappings for other V2 Instructions

        InstructionType::InitializeLbPair2 => {
            // Similar to V1 InitializeLbPair
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArrayBitmapExtension"); // Assuming it's still needed
            assign_if_exists(&mut role_by_index, 2, "tokenMintX");
            assign_if_exists(&mut role_by_index, 3, "tokenMintY");
            assign_if_exists(&mut role_by_index, 4, "reserveX");
            assign_if_exists(&mut role_by_index, 5, "reserveY");
            assign_if_exists(&mut role_by_index, 6, "oracle");
            // presetParameter might differ or be optional in V2, check IDL if issues arise
            assign_if_exists(&mut role_by_index, 7, "funder"); // Changed from presetParameter in V1
            assign_if_exists(&mut role_by_index, 8, "tokenProgram"); // Typically Token Program (or Token-2022)
            assign_if_exists(&mut role_by_index, 9, "systemProgram");
            assign_if_exists(&mut role_by_index, 10, "rent");
            assign_if_exists(&mut role_by_index, 11, "eventAuthority");
            assign_if_exists(&mut role_by_index, 12, "program");
        },

        InstructionType::InitializeCustomizablePermissionlessLbPair2 => {
            // Complex instruction, mapping base accounts similar to InitializeLbPair2
            // Needs verification against exact IDL account list if possible
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "binArrayBitmapExtension");
            assign_if_exists(&mut role_by_index, 2, "tokenMintX");
            assign_if_exists(&mut role_by_index, 3, "tokenMintY");
            assign_if_exists(&mut role_by_index, 4, "reserveX");
            assign_if_exists(&mut role_by_index, 5, "reserveY");
            assign_if_exists(&mut role_by_index, 6, "oracle");
            assign_if_exists(&mut role_by_index, 7, "funder");
            assign_if_exists(&mut role_by_index, 8, "tokenProgram");
            assign_if_exists(&mut role_by_index, 9, "tokenProgram2022"); // Potentially needed
            assign_if_exists(&mut role_by_index, 10, "systemProgram");
            assign_if_exists(&mut role_by_index, 11, "rent");
            assign_if_exists(&mut role_by_index, 12, "eventAuthority");
            assign_if_exists(&mut role_by_index, 13, "program");
        },

        InstructionType::ClaimFee2 => {
            // Similar to V1 ClaimFee
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "position");
            assign_if_exists(&mut role_by_index, 2, "binArrayLower");
            assign_if_exists(&mut role_by_index, 3, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 4, "sender");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "userTokenX");
            assign_if_exists(&mut role_by_index, 8, "userTokenY");
            assign_if_exists(&mut role_by_index, 9, "tokenXMint");
            assign_if_exists(&mut role_by_index, 10, "tokenYMint");
            // Token programs likely passed via RemainingAccountsInfo for transfer hooks
            assign_if_exists(&mut role_by_index, 11, "eventAuthority");
            assign_if_exists(&mut role_by_index, 12, "program");
        },

        InstructionType::ClaimReward2 => {
            // Similar to V1 ClaimReward
            assign_if_exists(&mut role_by_index, 0, "lbPair");
            assign_if_exists(&mut role_by_index, 1, "position");
            assign_if_exists(&mut role_by_index, 2, "binArrayLower");
            assign_if_exists(&mut role_by_index, 3, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 4, "owner");
            assign_if_exists(&mut role_by_index, 5, "rewardVault");
            assign_if_exists(&mut role_by_index, 6, "rewardMint");
            assign_if_exists(&mut role_by_index, 7, "userTokenAccount");
            // Token program likely passed via RemainingAccountsInfo for transfer hooks
            assign_if_exists(&mut role_by_index, 8, "eventAuthority");
            assign_if_exists(&mut role_by_index, 9, "program");
        },

        InstructionType::AddLiquidity2 | 
        InstructionType::AddLiquidityByStrategy2 | 
        InstructionType::AddLiquidityOneSidePrecise2 => {
            // Similar to V1 AddLiquidity variants
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension"); // If applicable
            assign_if_exists(&mut role_by_index, 3, "userTokenX"); // Or userToken for one-side
            assign_if_exists(&mut role_by_index, 4, "userTokenY"); // Or userToken for one-side
            assign_if_exists(&mut role_by_index, 5, "reserveX");   // Or reserve for one-side
            assign_if_exists(&mut role_by_index, 6, "reserveY");   // Or reserve for one-side
            assign_if_exists(&mut role_by_index, 7, "tokenXMint"); // Or tokenMint for one-side
            assign_if_exists(&mut role_by_index, 8, "tokenYMint"); // Or tokenMint for one-side
            assign_if_exists(&mut role_by_index, 9, "binArrayLower");
            assign_if_exists(&mut role_by_index, 10, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 11, "sender"); // Or owner
            // Token programs likely passed via RemainingAccountsInfo for transfer hooks
            assign_if_exists(&mut role_by_index, 12, "eventAuthority");
            assign_if_exists(&mut role_by_index, 13, "program");
        },

        InstructionType::RemoveLiquidity2 | 
        InstructionType::RemoveLiquidityByRange2 => {
            // Similar to V1 RemoveLiquidity variants
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayBitmapExtension"); // If applicable
            assign_if_exists(&mut role_by_index, 3, "userTokenX");
            assign_if_exists(&mut role_by_index, 4, "userTokenY");
            assign_if_exists(&mut role_by_index, 5, "reserveX");
            assign_if_exists(&mut role_by_index, 6, "reserveY");
            assign_if_exists(&mut role_by_index, 7, "tokenXMint");
            assign_if_exists(&mut role_by_index, 8, "tokenYMint");
            assign_if_exists(&mut role_by_index, 9, "binArrayLower");
            assign_if_exists(&mut role_by_index, 10, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 11, "sender"); // Or owner
            // Token programs likely passed via RemainingAccountsInfo for transfer hooks
            assign_if_exists(&mut role_by_index, 12, "eventAuthority");
            assign_if_exists(&mut role_by_index, 13, "program");
        },

        InstructionType::ClosePosition2 => {
            // Similar to V1 ClosePosition
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "lbPair");
            assign_if_exists(&mut role_by_index, 2, "binArrayLower");
            assign_if_exists(&mut role_by_index, 3, "binArrayUpper");
            assign_if_exists(&mut role_by_index, 4, "sender"); // Or owner
            assign_if_exists(&mut role_by_index, 5, "rentReceiver");
            assign_if_exists(&mut role_by_index, 6, "eventAuthority");
            assign_if_exists(&mut role_by_index, 7, "program");
        },

        // UpdateFeesAndReward2 is handled by UpdateFeesAndRewards mapping

        InstructionType::ClosePositionIfEmpty => {
            // Guessing accounts based on name
            assign_if_exists(&mut role_by_index, 0, "position");
            assign_if_exists(&mut role_by_index, 1, "owner");
            assign_if_exists(&mut role_by_index, 2, "rentReceiver");
            assign_if_exists(&mut role_by_index, 3, "systemProgram"); // Often needed for closing
            assign_if_exists(&mut role_by_index, 4, "eventAuthority");
            assign_if_exists(&mut role_by_index, 5, "program");
        },

        InstructionType::InitializeTokenBadge => {
            // Guessing accounts based on name
            assign_if_exists(&mut role_by_index, 0, "tokenBadge"); // The account being initialized
            assign_if_exists(&mut role_by_index, 1, "lbPair");     // Likely associated with a pair
            assign_if_exists(&mut role_by_index, 2, "payer");      // Who pays for init
            assign_if_exists(&mut role_by_index, 3, "systemProgram");
            assign_if_exists(&mut role_by_index, 4, "rent");
            assign_if_exists(&mut role_by_index, 5, "eventAuthority");
            assign_if_exists(&mut role_by_index, 6, "program");
        },
        InstructionType::CreateClaimProtocolFeeOperator => {
            // Guessing accounts
            assign_if_exists(&mut role_by_index, 0, "claimOperator"); // Account being created
            assign_if_exists(&mut role_by_index, 1, "admin");         // Authority
            assign_if_exists(&mut role_by_index, 2, "payer");
            assign_if_exists(&mut role_by_index, 3, "systemProgram");
            assign_if_exists(&mut role_by_index, 4, "rent");
            assign_if_exists(&mut role_by_index, 5, "eventAuthority");
            assign_if_exists(&mut role_by_index, 6, "program");
        },
        InstructionType::CloseClaimProtocolFeeOperator => {
            // Guessing accounts
            assign_if_exists(&mut role_by_index, 0, "claimOperator"); // Account being closed
            assign_if_exists(&mut role_by_index, 1, "admin");         // Authority
            assign_if_exists(&mut role_by_index, 2, "rentReceiver");
            assign_if_exists(&mut role_by_index, 3, "systemProgram");
            assign_if_exists(&mut role_by_index, 4, "eventAuthority");
            assign_if_exists(&mut role_by_index, 5, "program");
        },

        // Event logs typically don't have associated accounts passed in the instruction
        // itself (they might reference accounts involved in the original action).
        // We return an empty mapping for EventLog.
        InstructionType::EventLog => {
            // No accounts associated with EventLog instruction itself
        },
        
        // Add more instruction types as needed
        _ => {
            // Default to generic account labels for unmapped instructions
            for idx in 0..account_indices.len() {
                role_by_index.insert(idx, format!("account_{}", idx));
            }
        }
    }
    
    // Convert index-based mapping to account-based vector and return account roles in the same order as accounts
    let mut accounts = Vec::new();
    for idx in 0..account_indices.len() {
        let role = role_by_index.get(&idx).cloned().unwrap_or_else(|| format!("account_{}", idx));
        accounts.push(role);
    }
    
    // We're returning an empty input_accounts map here - this will be properly filled
    // in the process_instruction function where we have access to the account addresses
    let input_accounts = HashMap::new();
    
    InputAccounts { 
        accounts,
        input_accounts,
    }
}

// Helper function to assign an account role if it exists at a given index
fn assign_if_exists(roles: &mut HashMap<usize, String>, idx: usize, role: &str) {
    roles.insert(idx, role.to_string());
}

// Helper function for instructions with no specific accounts (like EventLog)
pub fn map_empty_accounts() -> InputAccounts {
    InputAccounts {
        accounts: Vec::new(),
        input_accounts: HashMap::new(),
    }
}

// Helper function for generic account mapping if specific mapping not defined
// Used as a fallback or for Unknown instructions 