// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           10
// Async Callback (empty):               1
// Total number of exported functions:  12

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    bonding_curve_contract
    (
        init => init
        sellToken => sell_token_endpoint
        buyToken => buy_token_endpoint
        deposit => deposit_endpoint
        setBondingCurve => set_bonding_curve_endpoint
        claim => claim_endpoint
        view_buy_price => view_buy_price
        view_sell_price => view_sell_price
        getTokenAvailability => get_token_availability
        setLocalRoles => set_local_roles
        unsetLocalRoles => unset_local_roles
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
