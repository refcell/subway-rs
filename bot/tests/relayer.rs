use ethers::prelude::*;
use ethers_flashbots::{BundleTransaction, SimulatedBundle, SimulatedTransaction};
use hex::FromHex;
use std::str::FromStr;

use subway_rs::relayer;

#[test]
fn test_validate_simulation_response() {
    let mut simulated_bundle: SimulatedBundle = SimulatedBundle {
        hash: H256::from_str("5bc4a7c668569e0a54a23f04afd879f923d60064c10ec2818c351eda3b69d0a8")
            .unwrap(),
        coinbase_diff: U256::from_dec_str("1").unwrap(),
        coinbase_tip: U256::from_dec_str("1").unwrap(),
        gas_price: U256::from_dec_str("1").unwrap(),
        gas_used: U256::from_dec_str("1").unwrap(),
        gas_fees: U256::from_dec_str("1").unwrap(),
        simulation_block: U64::from(1),
        transactions: vec![],
    };
    assert_eq!(simulated_bundle.effective_gas_price(), U256::from(1));

    // Test validation with successful transactions
    simulated_bundle.transactions = vec![SimulatedTransaction {
        hash: H256::from_str("5bc4a7c668569e0a54a23f04afd879f923d60064c10ec2818c351eda3b69d0a8")
            .unwrap(),
        coinbase_diff: U256::from_dec_str("1").unwrap(),
        coinbase_tip: U256::from_dec_str("1").unwrap(),
        gas_price: U256::from_dec_str("1").unwrap(),
        gas_used: U256::from_dec_str("1").unwrap(),
        gas_fees: U256::from_dec_str("1").unwrap(),
        from: Address::from_str("0x6d51e250434a2b37e391c445c42f19c53c6bbdb4").unwrap(),
        to: Some(Address::from_str("0x6d51e250434a2b37e391c445c42f19c53c6bbdb4").unwrap()),
        value: None,
        error: None,
        revert: None,
    }];
    let _ = relayer::validate_simulation_response(&simulated_bundle).unwrap();

    // Test validation with error transaction
    simulated_bundle.transactions = vec![SimulatedTransaction {
        hash: H256::from_str("5bc4a7c668569e0a54a23f04afd879f923d60064c10ec2818c351eda3b69d0a8")
            .unwrap(),
        coinbase_diff: U256::from_dec_str("1").unwrap(),
        coinbase_tip: U256::from_dec_str("1").unwrap(),
        gas_price: U256::from_dec_str("1").unwrap(),
        gas_used: U256::from_dec_str("1").unwrap(),
        gas_fees: U256::from_dec_str("1").unwrap(),
        from: Address::from_str("0x6d51e250434a2b37e391c445c42f19c53c6bbdb4").unwrap(),
        to: Some(Address::from_str("0x6d51e250434a2b37e391c445c42f19c53c6bbdb4").unwrap()),
        value: None,
        error: Some(String::from("500: Internal Flashbots Relay Error")),
        revert: None,
    }];
    match relayer::validate_simulation_response(&simulated_bundle) {
        Ok(_) => panic!("Expected validation error for simulated bundle!"),
        Err(_) => {}
    }

    // Test validation with reverting transaction
    simulated_bundle.transactions = vec![SimulatedTransaction {
        hash: H256::from_str("5bc4a7c668569e0a54a23f04afd879f923d60064c10ec2818c351eda3b69d0a8")
            .unwrap(),
        coinbase_diff: U256::from_dec_str("1").unwrap(),
        coinbase_tip: U256::from_dec_str("1").unwrap(),
        gas_price: U256::from_dec_str("1").unwrap(),
        gas_used: U256::from_dec_str("1").unwrap(),
        gas_fees: U256::from_dec_str("1").unwrap(),
        from: Address::from_str("0x6d51e250434a2b37e391c445c42f19c53c6bbdb4").unwrap(),
        to: Some(Address::from_str("0x6d51e250434a2b37e391c445c42f19c53c6bbdb4").unwrap()),
        value: None,
        error: None,
        revert: Some(String::from("Revert: Arithmetic Overflow/Underflow")),
    }];
    match relayer::validate_simulation_response(&simulated_bundle) {
        Ok(_) => panic!("Expected validation error for simulated bundle!"),
        Err(_) => {}
    }
}

#[test]
fn test_construct_bundle_raw_transactions() {
    // Example Signed Raw Transaction
    let raw_data: String = "0x02f90312010b845b31f28085037e11d60083061a809400000000009726632680fb29d3f7a9734e3010e280b902a4b0480bbd0000000000000000000000006b175474e89094c44da98b954eedeac495271d0f000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000001111111254fb6c44bac0bed2854e76f90643097d00000000000000000000000000000000000000000000000000000000000001a00000000000000000000000000000000000000000000000007efc27e09bddb994000000000000000000000000000000000000000000000000011451bf389ef0cf000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000062d9afa80000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000001be231a5933e022bd0ce375c26e46cb23cb39b387c9c38d904d3fca0082974582e3ece4f17c1ac8bffe8a1a8a22bbf34c35d626acd545199e8ac01793981f89fab00000000000000000000000000000000000000000000000000000000000000c82e95b6c80000000000000000000000006b175474e89094c44da98b954eedeac495271d0f0000000000000000000000000000000000000000000000007de7d621633ec8c5000000000000000000000000000000000000000000000000000000000088b4b40000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000100000000000000003b6d0340ae461ca67b15dc8dc81ce7615e0320da1a9ab8d5520b7e0f000000000000000000000000000000000000000000000000c080a05fafb325d3aab047a5ff37be215329b967c9f7e09df44583e10d5c9667e76163a07c76bd102451ba27ef1d161b243a9c8f4b1ee88707202038a650aceae0809cc2".to_string();
    let prefix_stripped = raw_data.strip_prefix("0x").unwrap_or(raw_data.as_str());
    let raw_bytes = Bytes::from(Vec::from_hex(prefix_stripped).unwrap_or_default());
    let signed_transactions = vec![raw_bytes.clone()];

    // Construct the bundle
    let constructed = relayer::construct_bundle(signed_transactions, U64::from(1)).unwrap();
    assert_eq!(constructed.block(), Some(U64::from(2)));
    assert_eq!(constructed.simulation_block(), Some(U64::from(1)));
    assert_eq!(constructed.simulation_basefee(), None);
    assert_eq!(
        constructed.transaction_hashes(),
        vec![
            H256::from_str("0x7ec8a7ab42c06e02b6f9a9beffa048bf2139f9e21a9342cf10d7b1de6301b1fc")
                .unwrap()
        ]
    );
    match &constructed.transactions()[0] {
        BundleTransaction::Raw(b) => assert_eq!(*b, raw_bytes),
        BundleTransaction::Signed(tx) => panic!("Did not expected signed tx: {:?}", tx),
    }
}

#[test]
fn test_construct_bundle_signed_transactions() {
    // Construct a signed transaction
    let tx_hash =
        H256::from_str("5bc4a7c668569e0a54a23f04afd879f923d60064c10ec2818c351eda3b69d0a8").unwrap();
    let signed_transactions = vec![ethers::types::Transaction {
        hash: tx_hash,
        nonce: U256::from_dec_str("3").unwrap(),
        block_hash: None,
        block_number: None,
        transaction_index: None,
        from: H160::from_str("0x6d51e250434a2b37e391c445c42f19c53c6bbdb4").unwrap(),
        to: Some(H160::from_str("0x00000000009726632680fb29d3f7a9734e3010e2").unwrap()),
        value: U256::zero(),
        gas_price: None,
        gas: U256::from_str_radix("600000", 10).unwrap(),
        input: Bytes::default(),
        v: U64::from_dec_str("1").unwrap(),
        r: U256::from_str_radix(
            "13221257997149115367554116484997994517811841715428204585299610590519873770176",
            10,
        )
        .unwrap(),
        s: U256::from_str_radix(
            "4031628591960627198678530000218440803766672183405718256126725182044375106764",
            10,
        )
        .unwrap(),
        transaction_type: Some(U64::from_dec_str("2").unwrap()),
        access_list: Some(Default::default()),
        max_priority_fee_per_gas: Some(U256::from_str_radix("1000000000", 10).unwrap()),
        max_fee_per_gas: Some(U256::from_str_radix("2000000000", 10).unwrap()),
        chain_id: Some(U256::from_dec_str("1").unwrap()),
        other: Default::default(),
    }];

    // Construct the bundle
    let constructed = relayer::construct_bundle(signed_transactions, U64::from(1)).unwrap();
    assert_eq!(constructed.block(), Some(U64::from(2)));
    assert_eq!(constructed.simulation_block(), Some(U64::from(1)));
    assert_eq!(constructed.simulation_basefee(), None);
    assert_eq!(constructed.transaction_hashes()[0], tx_hash);
    match &constructed.transactions()[0] {
        BundleTransaction::Raw(_) => panic!("Expected signed transaction, not raw!"),
        BundleTransaction::Signed(tx) => assert_eq!(tx.hash, tx_hash),
    }
}
