use cosmwasm_std::{Addr, Timestamp};
use cw_multi_test::Executor;
use sg2::tests::mock_collection_params_1;
use sg_std::GENESIS_MINT_START_TIME;

use crate::common_setup::contract_boxes::custom_mock_app;
use crate::common_setup::msg::MinterCollectionResponse;
use crate::common_setup::setup_accounts_and_block::{setup_accounts, setup_block_time};
use crate::common_setup::setup_collection_whitelist::{
    configure_collection_whitelist, setup_whitelist_contract,
};
use crate::common_setup::setup_minter::{configure_minter, minter_params_token};
use crate::sg_eth_airdrop::constants::claim_constants::{CONFIG_PLAINTEXT, STARGAZE_WALLET_01};
use crate::sg_eth_airdrop::constants::collection_constants::{
    AIRDROP_ADDR_STR, MINT_PRICE, WHITELIST_AMOUNT,
};
use crate::sg_eth_airdrop::setup::collection_whitelist_helpers::{
    execute_airdrop_claim, execute_mint_fail_not_on_whitelist, execute_mint_success,
    send_funds_to_address, update_admin_for_whitelist,
};
use crate::sg_eth_airdrop::setup::execute_msg::instantiate_contract;
use crate::sg_eth_airdrop::setup::setup_signatures::{get_msg_plaintext, get_wallet_and_sig};
use crate::sg_eth_airdrop::setup::test_msgs::InstantiateParams;
use sg_eth_airdrop::msg::QueryMsg;
extern crate whitelist_immutable;
use sg_eth_airdrop::contract::INSTANTIATION_FEE;

#[test]
fn test_set_minter_contract_success() {
    let mut app = custom_mock_app();
    let (creator, _) = setup_accounts(&mut app);
    let start_time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
    let collection_params = mock_collection_params_1(Some(start_time));
    let minter_params = minter_params_token(1);
    let minter_collection_response: Vec<MinterCollectionResponse> = configure_minter(
        &mut app,
        creator.clone(),
        vec![collection_params],
        vec![minter_params],
    );
    let minter_addr = minter_collection_response[0].minter.clone().unwrap();

    let claim_plaintext = &get_msg_plaintext(STARGAZE_WALLET_01.to_string());
    let (_, _, _, eth_addr_str) = get_wallet_and_sig(claim_plaintext.clone());

    let contract_admin = Addr::unchecked(creator);
    let params = InstantiateParams {
        addresses: vec![eth_addr_str],
        funds_amount: WHITELIST_AMOUNT + INSTANTIATION_FEE,
        expected_airdrop_contract_id: 4,
        minter_address: minter_addr.clone(),
        admin_account: contract_admin,
        app: &mut app,
        per_address_limit: 1,
        claim_msg_plaintext: CONFIG_PLAINTEXT.to_string(),
    };
    instantiate_contract(params).unwrap();
    let airdrop_contract = Addr::unchecked("contract3");
    let query_msg = QueryMsg::GetMinter {};
    let result: Addr = app
        .wrap()
        .query_wasm_smart(airdrop_contract, &query_msg)
        .unwrap();
    assert_eq!(result, minter_addr);
}

#[test]
fn test_claim_added_to_minter_whitelist() {
    let mut app = custom_mock_app();
    let (creator, buyer) = setup_accounts(&mut app);
    let start_time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
    let collection_params = mock_collection_params_1(Some(start_time));
    let minter_params = minter_params_token(1);
    let minter_collection_response: Vec<MinterCollectionResponse> = configure_minter(
        &mut app,
        creator.clone(),
        vec![collection_params],
        vec![minter_params],
    );
    let minter_addr = minter_collection_response[0].minter.clone().unwrap();
    let whitelist_addr = configure_collection_whitelist(
        &mut app,
        creator.clone(),
        buyer.clone(),
        minter_addr.clone(),
    );
    setup_block_time(&mut app, GENESIS_MINT_START_TIME, None);
    let claim_plaintext = &get_msg_plaintext(STARGAZE_WALLET_01.to_string());
    let (_, eth_sig_str, _, eth_addr_str) = get_wallet_and_sig(claim_plaintext.clone());

    let airdrop_contract = Addr::unchecked(AIRDROP_ADDR_STR);
    let params = InstantiateParams {
        addresses: vec![eth_addr_str.clone()],
        funds_amount: WHITELIST_AMOUNT + INSTANTIATION_FEE,
        expected_airdrop_contract_id: 5,
        minter_address: minter_addr.clone(),
        admin_account: creator.clone(),
        app: &mut app,
        per_address_limit: 1,
        claim_msg_plaintext: CONFIG_PLAINTEXT.to_string(),
    };
    instantiate_contract(params).unwrap();

    let stargaze_wallet_01 = Addr::unchecked(STARGAZE_WALLET_01);
    update_admin_for_whitelist(&mut app, creator, airdrop_contract.clone(), whitelist_addr);
    send_funds_to_address(&mut app, STARGAZE_WALLET_01, MINT_PRICE);
    execute_mint_fail_not_on_whitelist(&mut app, minter_addr.clone());
    execute_airdrop_claim(
        &mut app,
        eth_addr_str,
        eth_sig_str,
        stargaze_wallet_01.clone(),
        airdrop_contract,
    );
    execute_mint_success(&mut app, stargaze_wallet_01, minter_addr);
}
