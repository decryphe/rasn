// Test whether constrained OctetString and FixedOctetString are equal
use bitvec::prelude::*;
use rasn::prelude::*;
use rasn::{ber, jer, oer, uper};

#[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq)]
#[rasn(automatic_tags)]
pub struct Hashes {
    #[rasn(size("3"))]
    pub hashed3: OctetString,
    #[rasn(size("8"))]
    pub hashed8: OctetString,
    #[rasn(size("16"))]
    pub hashed16: OctetString,
    #[rasn(size("32"))]
    pub hashed32: OctetString,
    #[rasn(size("64"))]
    pub hashed64: OctetString,
}
#[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq)]
#[rasn(automatic_tags)]
pub struct ConstrainedHashes {
    pub hashed3: FixedOctetString<3>,
    pub hashed8: FixedOctetString<8>,
    pub hashed16: FixedOctetString<16>,
    pub hashed32: FixedOctetString<32>,
    pub hashed64: FixedOctetString<64>,
}
#[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq)]
#[rasn(automatic_tags)]
pub struct ConstrainedFixBitString {
    pub hashed3: FixedBitString<3>,
}
#[derive(AsnType, Decode, Encode, Debug, Clone, PartialEq)]
#[rasn(automatic_tags)]
pub struct ConstrainedBitString {
    #[rasn(size("3"))]
    pub hashed3: BitString,
}

fn build_octet() -> Hashes {
    Hashes {
        hashed3: vec![0x01, 0x02, 0x03].into(),
        hashed8: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08].into(),
        hashed16: vec![
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ]
        .into(),
        hashed32: vec![
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ]
        .into(),
        hashed64: vec![
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A,
            0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
            0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40,
        ]
        .into(),
    }
}
fn build_fixed_octet() -> ConstrainedHashes {
    ConstrainedHashes {
        hashed3: [0x01, 0x02, 0x03].into(),
        hashed8: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08].into(),
        hashed16: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ]
        .into(),
        hashed32: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ]
        .into(),
        hashed64: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A,
            0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
            0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40,
        ]
        .into(),
    }
}

macro_rules! test_decode_eq {
    ($fn_name:ident, $codec:ident) => {
        #[test]
        fn $fn_name() {
            let items = build_octet();
            let fixed_items = build_fixed_octet();
            let encoded = $codec::encode(&items).unwrap();
            let encoded_fixed = $codec::encode(&fixed_items).unwrap();
            let decoded = $codec::decode::<Hashes>(&encoded).unwrap();
            let decoded_fixed = $codec::decode::<ConstrainedHashes>(&encoded_fixed).unwrap();
            let bits = BitString::from_bitslice(&BitVec::<u8, Msb0>::repeat(true, 3));
            let bs = ConstrainedBitString {
                hashed3: BitString::from(bits),
            };
            let bool_array = [true, true, true];
            let mut bit_array: BitArray<[u8; 3], Msb0> = BitArray::ZERO;
            for (i, &value) in bool_array.iter().enumerate() {
                bit_array.set(i, value);
            }
            let bs_fixed = ConstrainedFixBitString { hashed3: bit_array };
            let encoded_bs = $codec::encode(&bs).unwrap();
            let encoded_bs_fixed = $codec::encode(&bs_fixed).unwrap();
            let decoded_bs = $codec::decode::<ConstrainedBitString>(&encoded_bs).unwrap();
            let decoded_bs_fixed =
                $codec::decode::<ConstrainedFixBitString>(&encoded_bs_fixed).unwrap();
            assert_eq!(bs, decoded_bs);
            assert_eq!(encoded_bs, encoded_bs_fixed);
            assert_eq!(bs_fixed, decoded_bs_fixed);
            assert_eq!(items, decoded);
            assert_eq!(encoded, encoded_fixed);
            assert_eq!(fixed_items, decoded_fixed);
            assert_eq!(items.hashed3.as_ref(), decoded_fixed.hashed3.as_ref());
            assert_eq!(items.hashed8.as_ref(), decoded_fixed.hashed8.as_ref());
            assert_eq!(items.hashed16.as_ref(), decoded_fixed.hashed16.as_ref());
            assert_eq!(items.hashed32.as_ref(), decoded_fixed.hashed32.as_ref());
            assert_eq!(items.hashed64.as_ref(), decoded_fixed.hashed64.as_ref());
        }
    };
}
test_decode_eq!(test_uper_octet_eq, uper);
test_decode_eq!(test_oer_octet_eq, oer);
test_decode_eq!(test_ber_octet_eq, ber);
test_decode_eq!(test_jer_octet_eq, jer);
