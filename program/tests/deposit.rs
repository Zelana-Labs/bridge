mod common;
use bridge_z::{helpers::StateDefinition, instruction::{BridgeIx, DepositParams}, state::DepositReceipt, ID};
use common::TestFixture;
use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signer::Signer, system_program};

#[test]
fn text_deposit_success(){
    let mut fixture  = TestFixture::new();
    fixture.initialize_bridge().expect("Bridge initialize fail");

    let depositer_keypair  = fixture.payer.insecure_clone();
    let depositor_pubkey = depositer_keypair.pubkey();
    let deposit_amount = 1_000_000_000;
    let nonce = 12345u64;

    let vault_balance_before = fixture.svm.get_balance(&fixture.vault_pda).unwrap();
    let depositor_balance_before = fixture.svm.get_balance(&depositor_pubkey).unwrap();
    
    let pubkey = Pubkey::from(ID);
    let nonce_le = nonce.to_le_bytes();
    let (receipt_pda,_) = Pubkey::find_program_address(
        &[
            DepositReceipt::SEED.as_bytes(),
            fixture.config_pda.as_ref(),
            depositor_pubkey.as_ref(),
            &nonce_le
        ],
        &pubkey
        );

    let ix_data = DepositParams{
            amount:deposit_amount,
            nonce
        };

    let mut instruction_data = vec![BridgeIx::DEPOSIT as u8];
    instruction_data.extend_from_slice(bytemuck::bytes_of(&ix_data));

    let accounts = vec![
    AccountMeta::new(depositor_pubkey, true),
    AccountMeta::new_readonly(fixture.config_pda, false),
    AccountMeta::new(fixture.vault_pda, false),
    AccountMeta::new(receipt_pda, false),
    AccountMeta::new_readonly(system_program::ID, false),
    ];

    let deposit_ix = Instruction{
    program_id:pubkey,
    accounts,
    data:instruction_data
    };

    let result = fixture.build_and_send_transaction(&[], vec![deposit_ix]);

    println!("{:?}",result);

    assert!(result.is_ok(),"deposit transaction failed {:?}",result.unwrap_err());
    let vault_balance_after = fixture.svm.get_balance(&fixture.vault_pda).unwrap();
    println!("{} {}",vault_balance_before+deposit_amount,vault_balance_after);
    assert_eq!(vault_balance_after, vault_balance_before + deposit_amount);
    
    let depositor_balance_after = fixture.svm.get_balance(&depositor_pubkey).unwrap();
    assert!(depositor_balance_after < depositor_balance_before);
    
    let receipt_account = fixture.svm.get_account(&receipt_pda).expect("Deposit receipt account not found");
    assert_eq!(receipt_account.owner, pubkey);
    assert_eq!(receipt_account.data.len(), DepositReceipt::LEN);    
    let receipt_state: &DepositReceipt = bytemuck::from_bytes(&receipt_account.data);
    assert_eq!(receipt_state.depositor, *depositor_pubkey.as_array());
    assert_eq!(receipt_state.amount, deposit_amount);
    assert_eq!(receipt_state.nonce, nonce);
    assert_ne!(receipt_state.bump, 0);
}


#[test]
fn test_deposit_replay_fails() {
    let mut fixture = TestFixture::new();
    fixture.initialize_bridge().expect("Bridge initialization failed");

    let depositor = &fixture.payer;
    let deposit_amount = 500_000_000; // 0.5 SOL
    let nonce = 999u64; // The nonce we will reuse.

    let pubkey = Pubkey::from(ID);
    let nonce_le = nonce.to_le_bytes();
    let (receipt_pda, _) = Pubkey::find_program_address(
        &[
            DepositReceipt::SEED.as_bytes(),
            fixture.config_pda.as_ref(),
            depositor.pubkey().as_ref(),
            &nonce_le,
        ],
        &pubkey,
    );

    let ix_data = DepositParams { amount: deposit_amount, nonce };
    let mut instruction_data = vec![BridgeIx::DEPOSIT as u8];
    instruction_data.extend_from_slice(bytemuck::bytes_of(&ix_data));

    let accounts = vec![
        AccountMeta::new(depositor.pubkey(), true),
        AccountMeta::new_readonly(fixture.config_pda, false),
        AccountMeta::new(fixture.vault_pda, false),
        AccountMeta::new(receipt_pda, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let deposit_ix = Instruction { program_id: pubkey, accounts, data: instruction_data };

    let first_result = fixture.build_and_send_transaction(&[], vec![deposit_ix.clone()]);
    assert!(first_result.is_ok(), "First deposit should have succeeded");

    let second_result = fixture.build_and_send_transaction(&[], vec![deposit_ix]);

    assert!(second_result.is_err(), "Second deposit (replay) should have failed");

    let tx_error = second_result.unwrap_err().err;
    // assert_eq!(tx_error, ProgramError::AccountAlreadyInitialized);
}

