use core::marker::PhantomData;

use crate::{
    api::{
        BlockchainApi, ManagedTypeApi, StorageReadApi, StorageReadApiImpl, StorageWriteApi,
        StorageWriteApiImpl,
    },
    storage_clear, storage_get, storage_get_len,
    types::{BoxedBytes, ManagedBuffer, ManagedBytesNestedDecodeInput, ManagedType},
    BytesArgLoader,
};
use alloc::vec::Vec;
use elrond_codec::{DecodeError, NestedDecode, TopDecode, TopDecodeInput};

use super::CallbackClosureMatcher;

/// Temporary solution until gas costs of the managed version are reduced.
/// Only contains logic for deserializing and for being used in the callback handling macros.
pub struct CallbackClosureUnmanagedArgs<M: ManagedTypeApi> {
    callback_name: BoxedBytes,
    closure_args: Vec<BoxedBytes>,
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> CallbackClosureUnmanagedArgs<M> {
    /// Used by callback_raw.
    pub fn new_empty() -> Self {
        CallbackClosureUnmanagedArgs {
            callback_name: BoxedBytes::empty(),
            closure_args: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn storage_load_and_clear<A: BlockchainApi + StorageReadApi + StorageWriteApi>(
        api: A,
    ) -> Option<Self> {
        let storage_key = super::callback_closure::cb_closure_storage_key::<A>();
        if storage_get_len(&storage_key) > 0 {
            let closure = storage_get(&storage_key);
            storage_clear(&storage_key);
            Some(closure)
        } else {
            None
        }
    }

    pub fn matcher<const CB_NAME_MAX_LENGTH: usize>(
        &self,
    ) -> CallbackClosureMatcher<CB_NAME_MAX_LENGTH> {
        CallbackClosureMatcher::new_from_unmanaged(self.callback_name.as_slice())
    }

    pub fn into_arg_loader(self) -> BytesArgLoader<M> {
        BytesArgLoader::new(self.closure_args)
    }
}

impl<M: ManagedTypeApi> TopDecode for CallbackClosureUnmanagedArgs<M> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let managed_buffer: ManagedBuffer<M> = ManagedBuffer::top_decode(input)?;

        let mut nested_buffer =
            ManagedBytesNestedDecodeInput::<M>::new(managed_buffer.to_boxed_bytes().into_box());
        let callback_name = BoxedBytes::dep_decode(&mut nested_buffer)?;
        let closure_args = Vec::<BoxedBytes>::dep_decode(&mut nested_buffer)?;
        Ok(CallbackClosureUnmanagedArgs {
            callback_name,
            closure_args,
            _phantom: PhantomData,
        })
    }
}
