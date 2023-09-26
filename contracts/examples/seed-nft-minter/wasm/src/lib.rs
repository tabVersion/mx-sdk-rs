// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            9
// Async Callback:                       1
// Total number of exported functions:  11

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    seed_nft_minter
    (
        init => init
        createNft => create_nft
        claimAndDistribute => claim_and_distribute
        getMarketplaces => marketplaces
        getNftCount => nft_count
        getDistributionRules => distribution_rules
        issueToken => issue_token
        buyNft => buy_nft
        getNftPrice => get_nft_price
        getNftTokenId => nft_token_id
    )
}

multiversx_sc_wasm_adapter::async_callback! { seed_nft_minter }
