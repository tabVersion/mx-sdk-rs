use elrond_wasm::api::{Handle, ManagedTypeApi};
use num_bigint::BigInt;

use crate::TxContext;

impl ManagedTypeApi for TxContext {
    fn managed_buffer_to_big_int_signed(&self, buffer_handle: Handle) -> Handle {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        let bytes = tx_output
            .managed_types
            .managed_buffer_map
            .get(buffer_handle);
        let bi = BigInt::from_signed_bytes_be(bytes.as_slice());
        tx_output.managed_types.big_int_map.insert_new_handle(bi)
    }

    fn big_int_to_managed_buffer_signed(&self, big_int_handle: Handle) -> Handle {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        let bi = tx_output.managed_types.big_int_map.get(big_int_handle);
        let bytes = bi.to_signed_bytes_be();
        tx_output
            .managed_types
            .managed_buffer_map
            .insert_new_handle(bytes)
    }
}
