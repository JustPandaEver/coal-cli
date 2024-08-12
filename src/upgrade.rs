use colored::*;
use solana_program::bpf_loader_upgradeable::upgrade; // Import the correct upgrade function
use solana_sdk::{pubkey::Pubkey, signer::Signer};
use spl_token::amount_to_ui_amount;

use crate::{
    cu_limits::CU_LIMIT_UPGRADE,
    send_and_confirm::ComputeBudget,
    utils::{amount_f64_to_u64_v1, ask_confirm},
    Miner, UpgradeArgs,
};

impl Miner {
    pub async fn upgrade(&self, args: UpgradeArgs) {
        let signer = &self.signer();
        let _beneficiary = self.get_or_initialize_ata().await; // Prefix with underscore
        let (_sender, _sender_balance) = self.get_ata_v1().await; // Prefix with underscore

        let amount_f64 = match args.amount {
            Some(f64) => f64,
            None => {
                println!(
                    "Defaulting to max amount of v1 Ore token in wallet: {}",
                    _sender_balance // Use _sender_balance
                );
                _sender_balance // Use _sender_balance
            }
        };
        let amount = amount_f64_to_u64_v1(amount_f64);
        let amount_ui = amount_to_ui_amount(amount, coal_api::consts::TOKEN_DECIMALS_V1);

        if !ask_confirm(
            format!(
                "\n You are about to upgrade {}. \n\nAre you sure you want to continue? [Y/n]",
                format!("{} ORE", amount_ui).bold(),
            )
            .as_str(),
        ) {
            return;
        }

        // Ensure these values are correctly set for your upgrade operation
        let program_id = &signer.pubkey(); // Replace with correct Pubkey if needed
        let upgrade_authority = &signer.pubkey(); // Replace with correct Pubkey if needed
        let upgradeable_loader_id = &Pubkey::new_unique(); // Replace with correct Pubkey if needed
        let program_data = &Pubkey::new_unique(); // Replace with correct Pubkey if needed

        let ix = upgrade(
            program_id,
            upgrade_authority,
            upgradeable_loader_id,
            program_data
        );

        match self
            .send_and_confirm(&[ix], ComputeBudget::Fixed(CU_LIMIT_UPGRADE), false)
            .await
        {
            Ok(_sig) => {}
            Err(err) => {
                println!("error: {}", err);
            }
        }
    }

    // Asserts that token account exists and gets balance
    async fn get_ata_v1(&self) -> (Pubkey, f64) {
        // Initialize client.
        let signer = self.signer();
        let client = self.rpc_client.clone();

        // Derive associated token address (for v1 account)
        let token_account_pubkey_v1 = spl_associated_token_account::get_associated_token_address(
            &signer.pubkey(),
            &coal_api::consts::MINT_ADDRESS, // Use MINT_ADDRESS
        );

        // Get token account balance
        let balance = match client.get_token_account(&token_account_pubkey_v1).await {
            Ok(None) => {
                panic!("v1 token account doesn't exist")
            }
            Ok(Some(token_account)) => match token_account.token_amount.ui_amount {
                Some(ui_amount) => ui_amount,
                None => {
                    panic!(
                        "Error parsing token account UI amount: {}",
                        token_account.token_amount.amount
                    )
                }
            },
            Err(err) => {
                panic!("Error fetching token account: {}", err)
            }
        };

        // Return v1 token account address
        (token_account_pubkey_v1, balance)
    }

    async fn get_or_initialize_ata(&self) -> Pubkey {
        // Initialize client
        let signer = self.signer();
        let client = self.rpc_client.clone();

        // Derive associated token address (ata)
        let token_account_pubkey = spl_associated_token_account::get_associated_token_address(
            &signer.pubkey(),
            &coal_api::consts::MINT_ADDRESS, // Use MINT_ADDRESS
        );

        // Check if ata already exists or init
        if let Err(_err) = client.get_token_account(&token_account_pubkey).await {
            println!("Initializing v2 token account...");
            let ix = spl_associated_token_account::instruction::create_associated_token_account(
                &signer.pubkey(),
                &signer.pubkey(),
                &coal_api::consts::MINT_ADDRESS,
                &spl_token::id(),
            );
            self.send_and_confirm(&[ix], ComputeBudget::Fixed(500_000), false)
                .await
                .ok();
        }

        // Return token account address
        token_account_pubkey
    }
}
