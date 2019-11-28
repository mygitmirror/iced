/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::super::super::test_utils::from_str_conv::to_vec_u8;
use super::super::super::test_utils::*;
use super::super::super::*;
use super::test_utils::decoder_tests;
use std::collections::HashMap;

fn decoder_new_panics(bitness: i32) {
	let _ = Decoder::new(bitness, b"\x90", DecoderOptions::NONE);
}

#[test]
#[should_panic]
fn decoder_new_panics_0() {
	decoder_new_panics(0);
}

#[test]
#[should_panic]
fn decoder_new_panics_128() {
	decoder_new_panics(128);
}

#[test]
fn decode_multiple_instrs_with_one_instance() {
	let tests = decoder_tests(false, true);

	let mut bytes_map16: HashMap<(i32, u32), Vec<u8>> = HashMap::new();
	let mut bytes_map32: HashMap<(i32, u32), Vec<u8>> = HashMap::new();
	let mut bytes_map64: HashMap<(i32, u32), Vec<u8>> = HashMap::new();

	let mut map16: HashMap<(i32, u32), Decoder> = HashMap::new();
	let mut map32: HashMap<(i32, u32), Decoder> = HashMap::new();
	let mut map64: HashMap<(i32, u32), Decoder> = HashMap::new();

	for tc in &tests {
		let bytes_map = match tc.bitness() {
			16 => &mut bytes_map16,
			32 => &mut bytes_map32,
			64 => &mut bytes_map64,
			_ => unreachable!(),
		};
		let key = (tc.bitness(), tc.decoder_options());
		let vec = bytes_map.entry(key).or_insert(Default::default());
		let bytes = to_vec_u8(tc.hex_bytes()).unwrap();
		vec.extend(bytes);
	}

	for tc in &tests {
		let (bytes_map, map) = match tc.bitness() {
			16 => (&bytes_map16, &mut map16),
			32 => (&bytes_map32, &mut map32),
			64 => (&bytes_map64, &mut map64),
			_ => unreachable!(),
		};
		let key = (tc.bitness(), tc.decoder_options());
		let vec = bytes_map.get(&key).unwrap();
		let _ = map.entry(key).or_insert(Decoder::new(tc.bitness(), vec, tc.decoder_options()));
	}

	for tc in &tests {
		let bytes = to_vec_u8(tc.hex_bytes()).unwrap();
		let mut decoder = super::create_decoder(tc.bitness(), &bytes, tc.decoder_options()).0;
		let key = (tc.bitness(), tc.decoder_options());
		let decoder_all = match tc.bitness() {
			16 => map16.get_mut(&key).unwrap(),
			32 => map32.get_mut(&key).unwrap(),
			64 => map64.get_mut(&key).unwrap(),
			_ => unreachable!(),
		};
		let ip = decoder.ip();
		decoder_all.set_ip(ip);

		let index = decoder_all.data_index();
		let instr1 = decoder.decode();
		let mut instr2 = decoder_all.decode();
		let co1 = decoder.get_constant_offsets(&instr1);
		let co2 = decoder_all.get_constant_offsets(&instr2);
		assert_eq!(instr1.code(), instr2.code());
		decoder_all.set_data_index(index + bytes.len());
		if instr1.code() == Code::INVALID {
			// decoder_all has a bigger buffer and can decode more bytes
			instr2.set_len(bytes.len() as i32);
			instr2.set_next_ip(ip + bytes.len() as u64);
		}
		assert!(instr1.eq_all_bits(&instr2));
		assert!(instr2.eq_all_bits(&instr1));
		super::verify_constant_offsets(&co1, &co2);
	}
}

#[test]
fn data_index() {
	const BITNESS: i32 = 64;
	let bytes = b"\x23\x18\x62\x31\x7C\x8B\x11\xD3";
	let mut decoder = Decoder::new(BITNESS, bytes, DecoderOptions::NONE);
	decoder.set_ip(get_default_ip(BITNESS));

	assert!(decoder.can_decode());
	assert_eq!(0, decoder.data_index());
	assert_eq!(bytes.len(), decoder.max_data_index());

	let instr_a1 = decoder.decode();
	assert_eq!(Code::And_r32_rm32, instr_a1.code());

	assert!(decoder.can_decode());
	assert_eq!(2, decoder.data_index());
	assert_eq!(bytes.len(), decoder.max_data_index());

	let instr_b1 = decoder.decode();
	assert_eq!(Code::EVEX_Vmovups_xmmm128_k1z_xmm, instr_b1.code());

	assert!(!decoder.can_decode());
	assert_eq!(8, decoder.data_index());
	assert_eq!(bytes.len(), decoder.max_data_index());

	decoder.set_ip(get_default_ip(BITNESS) + 2);
	decoder.set_data_index(2);
	assert!(decoder.can_decode());
	assert_eq!(2, decoder.data_index());
	assert_eq!(bytes.len(), decoder.max_data_index());

	let instr_b2 = decoder.decode();
	assert_eq!(Code::EVEX_Vmovups_xmmm128_k1z_xmm, instr_b2.code());

	decoder.set_ip(get_default_ip(BITNESS) + 0);
	decoder.set_data_index(0);
	assert!(decoder.can_decode());
	assert_eq!(0, decoder.data_index());
	assert_eq!(bytes.len(), decoder.max_data_index());

	let instr_a2 = decoder.decode();
	assert_eq!(Code::And_r32_rm32, instr_a2.code());

	assert!(instr_a1.eq_all_bits(&instr_a2));
	assert!(instr_b1.eq_all_bits(&instr_b2));
}

#[test]
fn set_data_index_valid_index() {
	let bytes = b"\x23\x18\x62\x31\x7C\x8B\x11\xD3";
	let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
	for i in 0..(bytes.len() + 1) {
		decoder.set_data_index(i);
		assert_eq!(i, decoder.data_index());
	}
	let mut decoder = Decoder::new(64, b"", DecoderOptions::NONE);
	decoder.set_position(0);
	assert_eq!(0, decoder.position());
}

#[test]
#[should_panic]
fn set_data_index_panics_if_invalid() {
	let bytes = b"\x23\x18\x62\x31\x7C\x8B\x11\xD3";
	let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
	decoder.set_data_index(bytes.len() + 1);
}

#[test]
fn decoder_for_loop_into_iter() {
	let bytes = b"\x23\x18\x62\x31\x7C\x8B\x11\xD3";
	let decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
	let mut instrs: Vec<Instruction> = Vec::new();
	for instr in decoder {
		instrs.push(instr);
	}
	assert_eq!(2, instrs.len());
	assert_eq!(Code::And_r32_rm32, instrs[0].code());
	assert_eq!(Code::EVEX_Vmovups_xmmm128_k1z_xmm, instrs[1].code());
}

#[test]
fn decoder_for_loop_ref_mut_decoder() {
	let bytes = b"\x23\x18\x62\x31\x7C\x8B\x11\xD3";
	let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
	decoder.set_ip(0x1234_5678_9ABC_DEF0);
	let mut instrs: Vec<Instruction> = Vec::new();
	for instr in &mut decoder {
		instrs.push(instr);
	}
	assert_eq!(0x1234_5678_9ABC_DEF8, decoder.ip());
	assert_eq!(2, instrs.len());
	assert_eq!(Code::And_r32_rm32, instrs[0].code());
	assert_eq!(Code::EVEX_Vmovups_xmmm128_k1z_xmm, instrs[1].code());
}

#[test]
fn decoder_for_loop_decoder_iter() {
	let bytes = b"\x23\x18\x62\x31\x7C\x8B\x11\xD3";
	let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
	decoder.set_ip(0x1234_5678_9ABC_DEF0);
	let mut instrs: Vec<Instruction> = Vec::new();
	for instr in decoder.iter() {
		instrs.push(instr);
	}
	assert_eq!(0x1234_5678_9ABC_DEF8, decoder.ip());
	assert_eq!(2, instrs.len());
	assert_eq!(Code::And_r32_rm32, instrs[0].code());
	assert_eq!(Code::EVEX_Vmovups_xmmm128_k1z_xmm, instrs[1].code());
}

#[test]
fn decode_ip_xxxxxxxxffffffff() {
	let bytes = b"\x90";
	let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
	decoder.set_ip(0x1234_5678_FFFF_FFFF);
	let _ = decoder.decode();
	assert_eq!(0x1234_5679_0000_0000, decoder.ip());
}
