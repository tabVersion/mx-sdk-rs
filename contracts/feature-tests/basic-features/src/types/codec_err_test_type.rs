use elrond_wasm::derive::TypeAbi;
use elrond_wasm::elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

/// Helper type to explore encode/decode errors.
#[derive(TypeAbi)]
pub struct CodecErrorTestType;

impl TopEncode for CodecErrorTestType {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, _output: O) -> Result<(), EncodeError> {
        Err("deliberate top encode error".into())
    }
}

impl NestedEncode for CodecErrorTestType {
    fn dep_encode<O: NestedEncodeOutput>(&self, _dest: &mut O) -> Result<(), EncodeError> {
        Err("deliberate nested encode error".into())
    }
}

impl TopDecode for CodecErrorTestType {
    fn top_decode<I: TopDecodeInput>(_input: I) -> Result<Self, DecodeError> {
        Err("deliberate top decode error".into())
    }
}

impl NestedDecode for CodecErrorTestType {
    fn dep_decode<I: NestedDecodeInput>(_input: &mut I) -> Result<Self, DecodeError> {
        Err("deliberate nested decode error".into())
    }
}
