// Copyright 2015-2019 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hex_literal::hex;
use paste;
use crate::{encode, decode, Token, ParamType};

macro_rules! test_encode_decode {
	(name: $name:tt, types: $types:expr, tokens: $tokens:expr, data: $data:tt) => {
		paste::item! {
			#[test]
			fn [<encode_ $name>]() {
				let encoded = encode(&$tokens);
				let expected = hex_literal::hex!($data).to_vec();
				assert_eq!(encoded, expected);
			}

			#[test]
			fn [<decode_ $name>]() {
				let encoded = hex!($data);
				let expected = $tokens;
				let decoded = decode(&$types, &encoded).unwrap();
				assert_eq!(decoded, expected);
			}
		}
	}
}

// test address
test_encode_decode! {
	name: address,
	types: [ParamType::Address],
	tokens: [Token::Address([0x11u8; 20].into())],
	data: "0000000000000000000000001111111111111111111111111111111111111111"
}
test_encode_decode! {
	name: addresses,
	types: [
	  ParamType::Address,
	  ParamType::Address
	],
	tokens: [
	  Token::Address([0x11u8; 20].into()),
	  Token::Address([0x22u8; 20].into())
	],
	data: "
		0000000000000000000000001111111111111111111111111111111111111111
		0000000000000000000000002222222222222222222222222222222222222222"
}

// test bytes
test_encode_decode! {
	name: bytes,
	types: [ParamType::Bytes],
	tokens: [Token::Bytes(vec![0x12, 0x34])],
	data: "
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000002
		1234000000000000000000000000000000000000000000000000000000000000"
}
test_encode_decode! {
	name: bytes2,
	types: [ParamType::Bytes],
	tokens: [Token::Bytes(hex!("10000000000000000000000000000000000000000000000000000000000002").to_vec())],
	data: "
		0000000000000000000000000000000000000000000000000000000000000020
		000000000000000000000000000000000000000000000000000000000000001f
		1000000000000000000000000000000000000000000000000000000000000200"
}
test_encode_decode! {
	name: bytes3,
	types: [ParamType::Bytes],
	tokens: [
		Token::Bytes(hex!("
			1000000000000000000000000000000000000000000000000000000000000000
			1000000000000000000000000000000000000000000000000000000000000000
		").to_vec())
	],
	data: "
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000040
		1000000000000000000000000000000000000000000000000000000000000000
		1000000000000000000000000000000000000000000000000000000000000000"
}
test_encode_decode! {
	name: two_bytes,
	types: [ParamType::Bytes, ParamType::Bytes],
	tokens: [
		Token::Bytes(hex!("10000000000000000000000000000000000000000000000000000000000002").to_vec()),
		Token::Bytes(hex!("0010000000000000000000000000000000000000000000000000000000000002").to_vec())
	],
	data: "
		0000000000000000000000000000000000000000000000000000000000000040
		0000000000000000000000000000000000000000000000000000000000000080
		000000000000000000000000000000000000000000000000000000000000001f
		1000000000000000000000000000000000000000000000000000000000000200
		0000000000000000000000000000000000000000000000000000000000000020
		0010000000000000000000000000000000000000000000000000000000000002"
}

// test int
test_encode_decode! {
	name: int,
	types: [ParamType::Int(32)],
	tokens: [Token::Int([0x11u8; 32].into())],
	data: "1111111111111111111111111111111111111111111111111111111111111111"
}
test_encode_decode! {
	name: int2,
	types: [ParamType::Int(32)],
	tokens: {
		let mut int = [0u8; 32];
		int[31] = 4;
		[Token::Int(int.into())]
	},
	data: "0000000000000000000000000000000000000000000000000000000000000004"
}

// test uint
test_encode_decode! {
	name: uint,
	types: [ParamType::Uint(32)],
	tokens: [Token::Uint([0x11u8; 32].into())],
	data: "1111111111111111111111111111111111111111111111111111111111111111"
}
test_encode_decode! {
	name: uint2,
	types: [ParamType::Uint(32)],
	tokens: {
		let mut uint = [0u8; 32];
		uint[31] = 4;
		[Token::Uint(uint.into())]
	},
	data: "0000000000000000000000000000000000000000000000000000000000000004"
}

// test bool
test_encode_decode! {
	name: bool,
	types: [ParamType::Bool],
	tokens: [Token::Bool(true)],
	data: "0000000000000000000000000000000000000000000000000000000000000001"
}
test_encode_decode! {
	name: bool2,
	types: [ParamType::Bool],
	tokens: [Token::Bool(false)],
	data: "0000000000000000000000000000000000000000000000000000000000000000"
}

// test string
test_encode_decode! {
	name: string,
	types: [ParamType::String],
	tokens: [Token::String("gavofyork".to_owned())],
	data: "
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000009
		6761766f66796f726b0000000000000000000000000000000000000000000000"
}

// test array
test_encode_decode! {
	name: dynamic_array_of_addresses,
	types: [ParamType::Array(Box::new(ParamType::Address))],
	tokens: {
		let address1 = Token::Address([0x11u8; 20].into());
		let address2 = Token::Address([0x22u8; 20].into());
		[Token::Array(vec![address1, address2])]
	},
	data: "
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000001111111111111111111111111111111111111111
		0000000000000000000000002222222222222222222222222222222222222222"
}
test_encode_decode! {
	name: dynamic_array_of_fixed_arrays_of_addresses,
	types: [
		ParamType::Array(Box::new(
			ParamType::FixedArray(Box::new(ParamType::Address), 2)
		))
	],
	tokens: {
		let address1 = Token::Address([0x11u8; 20].into());
		let address2 = Token::Address([0x22u8; 20].into());
		let address3 = Token::Address([0x33u8; 20].into());
		let address4 = Token::Address([0x44u8; 20].into());
		let array0 = Token::FixedArray(vec![address1, address2]);
		let array1 = Token::FixedArray(vec![address3, address4]);
		[Token::Array(vec![array0, array1])]
	},
   	data: "
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000001111111111111111111111111111111111111111
		0000000000000000000000002222222222222222222222222222222222222222
		0000000000000000000000003333333333333333333333333333333333333333
		0000000000000000000000004444444444444444444444444444444444444444"
}
test_encode_decode! {
	name: dynamic_array_of_fixed_arrays_of_dynamic_array,
	types: [
		ParamType::Array(Box::new(
			ParamType::FixedArray(Box::new(ParamType::Array(Box::new(ParamType::Address))), 2)
		))
	],
	tokens: {
		let address1 = Token::Address([0x11u8; 20].into());
		let address2 = Token::Address([0x22u8; 20].into());
		let address3 = Token::Address([0x33u8; 20].into());
		let address4 = Token::Address([0x44u8; 20].into());
		let address5 = Token::Address([0x55u8; 20].into());
		let address6 = Token::Address([0x66u8; 20].into());
		let address7 = Token::Address([0x77u8; 20].into());
		let address8 = Token::Address([0x88u8; 20].into());
		let array0 = Token::FixedArray(vec![
			Token::Array(vec![address1, address2]),
			Token::Array(vec![address3, address4]),
		]);
		let array1 = Token::FixedArray(vec![
			Token::Array(vec![address5, address6]),
			Token::Array(vec![address7, address8]),
		]);
		[Token::Array(vec![array0, array1])]
	},
	data: "
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000000000000000000000000000000000000000000040
		0000000000000000000000000000000000000000000000000000000000000140
		0000000000000000000000000000000000000000000000000000000000000040
		00000000000000000000000000000000000000000000000000000000000000a0
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000001111111111111111111111111111111111111111
		0000000000000000000000002222222222222222222222222222222222222222
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000003333333333333333333333333333333333333333
		0000000000000000000000004444444444444444444444444444444444444444
		0000000000000000000000000000000000000000000000000000000000000040
		00000000000000000000000000000000000000000000000000000000000000a0
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000005555555555555555555555555555555555555555
		0000000000000000000000006666666666666666666666666666666666666666
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000007777777777777777777777777777777777777777
		0000000000000000000000008888888888888888888888888888888888888888"
}
test_encode_decode! {
	name: dynamic_array_of_dynamic_arrays,
	types: [
		ParamType::Array(Box::new(
			ParamType::Array(Box::new(ParamType::Address))
		))
	],
	tokens: {
		let address1 = Token::Address([0x11u8; 20].into());
		let address2 = Token::Address([0x22u8; 20].into());
		let array0 = Token::Array(vec![address1]);
		let array1 = Token::Array(vec![address2]);
		let dynamic = Token::Array(vec![array0, array1]);
		[dynamic]
	},
	data: "
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000000000000000000000000000000000000000000040
		0000000000000000000000000000000000000000000000000000000000000080
		0000000000000000000000000000000000000000000000000000000000000001
		0000000000000000000000001111111111111111111111111111111111111111
		0000000000000000000000000000000000000000000000000000000000000001
		0000000000000000000000002222222222222222222222222222222222222222"
}
test_encode_decode! {
	name: dynamic_array_of_dynamic_arrays2,
	types: [
		ParamType::Array(Box::new(
			ParamType::Array(Box::new(ParamType::Address))
		))
	],
	tokens: {
		let address1 = Token::Address([0x11u8; 20].into());
		let address2 = Token::Address([0x22u8; 20].into());
		let address3 = Token::Address([0x33u8; 20].into());
		let address4 = Token::Address([0x44u8; 20].into());
		let array0 = Token::Array(vec![address1, address2]);
		let array1 = Token::Array(vec![address3, address4]);
		let dynamic = Token::Array(vec![array0, array1]);
		[dynamic]
	},
	data: "
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000000000000000000000000000000000000000000040
		00000000000000000000000000000000000000000000000000000000000000a0
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000001111111111111111111111111111111111111111
		0000000000000000000000002222222222222222222222222222222222222222
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000003333333333333333333333333333333333333333
		0000000000000000000000004444444444444444444444444444444444444444"
}
test_encode_decode! {
	name: dynamic_array_of_bytes,
	types: [ParamType::Array(Box::new(ParamType::Bytes))],
	tokens: {
		let bytes = hex!("019c80031b20d5e69c8093a571162299032018d913930d93ab320ae5ea44a4218a274f00d607").to_vec();
		[Token::Array(vec![Token::Bytes(bytes)])]
	},
	data: "
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000001
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000026
		019c80031b20d5e69c8093a571162299032018d913930d93ab320ae5ea44a421
		8a274f00d6070000000000000000000000000000000000000000000000000000"
}
test_encode_decode! {
	name: dynamic_array_of_bytes2,
	types: [ParamType::Array(Box::new(ParamType::Bytes))],
	tokens: [
		Token::Array(vec![
			Token::Bytes(hex!("4444444444444444444444444444444444444444444444444444444444444444444444444444").to_vec()),
			Token::Bytes(hex!("6666666666666666666666666666666666666666666666666666666666666666666666666666").to_vec()),
		])
	],
	data: "
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000000000000000000000000000000000000000000040
		00000000000000000000000000000000000000000000000000000000000000a0
		0000000000000000000000000000000000000000000000000000000000000026
		4444444444444444444444444444444444444444444444444444444444444444
		4444444444440000000000000000000000000000000000000000000000000000
		0000000000000000000000000000000000000000000000000000000000000026
		6666666666666666666666666666666666666666666666666666666666666666
		6666666666660000000000000000000000000000000000000000000000000000"
}
test_encode_decode! {
	name: empty_dynamic_array,
	types: [
		ParamType::Array(Box::new(ParamType::Bool)),
		ParamType::Array(Box::new(ParamType::Bool)),
	],
	tokens: [
		Token::Array(vec![]),
		Token::Array(vec![])
	],
	data: "
		0000000000000000000000000000000000000000000000000000000000000040
		0000000000000000000000000000000000000000000000000000000000000060
		0000000000000000000000000000000000000000000000000000000000000000
		0000000000000000000000000000000000000000000000000000000000000000"
}
test_encode_decode! {
	name: dynamic_array_of_empty_dynamic_array,
	types: [
		ParamType::Array(Box::new(ParamType::Array(Box::new(ParamType::Bool)))),
		ParamType::Array(Box::new(ParamType::Array(Box::new(ParamType::Bool)))),
	],
	tokens: [
		Token::Array(vec![Token::Array(vec![])]),
		Token::Array(vec![Token::Array(vec![])]),
	],
	data: "
		0000000000000000000000000000000000000000000000000000000000000040
		00000000000000000000000000000000000000000000000000000000000000a0
		0000000000000000000000000000000000000000000000000000000000000001
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000000
		0000000000000000000000000000000000000000000000000000000000000001
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000000"
}

// test fixed array
test_encode_decode! {
	name: fixed_array_of_addresses,
	types: [ParamType::FixedArray(Box::new(ParamType::Address), 2)],
	tokens: {
		let address1 = Token::Address([0x11u8; 20].into());
		let address2 = Token::Address([0x22u8; 20].into());
		[Token::FixedArray(vec![address1, address2])]
	},
	data: "
		0000000000000000000000001111111111111111111111111111111111111111
		0000000000000000000000002222222222222222222222222222222222222222"
}
test_encode_decode! {
	name: fixed_array_of_fixed_arrays,
	types: [
		ParamType::FixedArray(
			Box::new(ParamType::FixedArray(Box::new(ParamType::Address), 2)),
			2
		)
	],
	tokens: {
		let address1 = Token::Address([0x11u8; 20].into());
		let address2 = Token::Address([0x22u8; 20].into());
		let address3 = Token::Address([0x33u8; 20].into());
		let address4 = Token::Address([0x44u8; 20].into());
		let array0 = Token::FixedArray(vec![address1, address2]);
		let array1 = Token::FixedArray(vec![address3, address4]);
		let fixed = Token::FixedArray(vec![array0, array1]);
		[fixed]
	},
	data: "
		0000000000000000000000001111111111111111111111111111111111111111
		0000000000000000000000002222222222222222222222222222222222222222
		0000000000000000000000003333333333333333333333333333333333333333
		0000000000000000000000004444444444444444444444444444444444444444"
}
test_encode_decode! {
	name: fixed_array_of_dynamic_array_of_addresses,
	types: [
		ParamType::FixedArray(
			Box::new(ParamType::Array(Box::new(ParamType::Address))),
			2
		)
	],
	tokens: {
		let address1 = Token::Address([0x11u8; 20].into());
		let address2 = Token::Address([0x22u8; 20].into());
		let address3 = Token::Address([0x33u8; 20].into());
		let address4 = Token::Address([0x44u8; 20].into());
		let array0 = Token::Array(vec![address1, address2]);
		let array1 = Token::Array(vec![address3, address4]);
		[Token::FixedArray(vec![array0, array1])]
	},
	data: "
		0000000000000000000000000000000000000000000000000000000000000020
		0000000000000000000000000000000000000000000000000000000000000040
		00000000000000000000000000000000000000000000000000000000000000a0
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000001111111111111111111111111111111111111111
		0000000000000000000000002222222222222222222222222222222222222222
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000003333333333333333333333333333333333333333
		0000000000000000000000004444444444444444444444444444444444444444"
}

// test fixed bytes
test_encode_decode! {
	name: fixed_bytes,
	types: [ParamType::FixedBytes(2)],
	tokens: [Token::FixedBytes(vec![0x12, 0x34])],
	data: "1234000000000000000000000000000000000000000000000000000000000000"
}

// comprehensive test
test_encode_decode! {
	name: comprehensive_test,
	types: [
		ParamType::Int(32),
		ParamType::Bytes,
		ParamType::Int(32),
		ParamType::Bytes,
	],
	tokens: {
		let bytes = hex!("
			131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
			131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
		").to_vec();
		[
			Token::Int(5.into()),
			Token::Bytes(bytes.clone()),
			Token::Int(3.into()),
			Token::Bytes(bytes),
		]
	},
	data: "
		0000000000000000000000000000000000000000000000000000000000000005
		0000000000000000000000000000000000000000000000000000000000000080
		0000000000000000000000000000000000000000000000000000000000000003
		00000000000000000000000000000000000000000000000000000000000000e0
		0000000000000000000000000000000000000000000000000000000000000040
		131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
		131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
		0000000000000000000000000000000000000000000000000000000000000040
		131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b
		131a3afc00d1b1e3461b955e53fc866dcf303b3eb9f4c16f89e388930f48134b"
}
test_encode_decode! {
	name: comprehensive_test2,
	types: [
		ParamType::Int(32),
		ParamType::String,
		ParamType::Int(32),
		ParamType::Int(32),
		ParamType::Int(32),
		ParamType::Array(Box::new(ParamType::Int(32))),
	],
	tokens: [
		Token::Int(1.into()),
		Token::String("gavofyork".to_owned()),
		Token::Int(2.into()),
		Token::Int(3.into()),
		Token::Int(4.into()),
		Token::Array(vec![
			Token::Int(5.into()),
			Token::Int(6.into()),
			Token::Int(7.into()),
		])
	],
	data: "
		0000000000000000000000000000000000000000000000000000000000000001
		00000000000000000000000000000000000000000000000000000000000000c0
		0000000000000000000000000000000000000000000000000000000000000002
		0000000000000000000000000000000000000000000000000000000000000003
		0000000000000000000000000000000000000000000000000000000000000004
		0000000000000000000000000000000000000000000000000000000000000100
		0000000000000000000000000000000000000000000000000000000000000009
		6761766f66796f726b0000000000000000000000000000000000000000000000
		0000000000000000000000000000000000000000000000000000000000000003
		0000000000000000000000000000000000000000000000000000000000000005
		0000000000000000000000000000000000000000000000000000000000000006
		0000000000000000000000000000000000000000000000000000000000000007"
}
