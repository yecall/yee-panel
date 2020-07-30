// Copyright (C) 2019 Yee Foundation.
//
// This file is part of YeeChain.
//
// YeeChain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// YeeChain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with YeeChain.  If not, see <https://www.gnu.org/licenses/>.

use parity_codec::Encode;
use parity_codec::{Compact, Decode};
use serde::export::TryFrom;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use substrate_primitives::blake2_256;
use substrate_primitives::storage::StorageKey;
use yee_signer::tx::call::Call;
use yee_signer::tx::types::{Era, Transaction};

use crate::rpc::errors;
use crate::rpc::serde::{Hex, SerdeHex};

#[allow(dead_code)]
pub type Hash = primitive_types::H256;

pub type BlockNumber = u64;

pub type Nonce = u64;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Header {
	#[serde(with = "SerdeHex")]
	pub extrinsics_root: Vec<u8>,

	#[serde(with = "SerdeHex")]
	pub parent_hash: Vec<u8>,

	#[serde(with = "SerdeHex")]
	pub state_root: Vec<u8>,

	#[serde(with = "SerdeHex")]
	pub number: BlockNumber,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockResponse {
	pub block: Block,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Block {
	pub header: Header,
	pub extrinsics: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultHeader {
	pub block_hash: Option<Hex<Vec<u8>>>,

	#[serde(with = "SerdeHex")]
	pub extrinsics_root: Vec<u8>,

	#[serde(with = "SerdeHex")]
	pub parent_hash: Vec<u8>,

	#[serde(with = "SerdeHex")]
	pub state_root: Vec<u8>,

	pub number: BlockNumber,
}

impl From<Header> for ResultHeader {
	fn from(t: Header) -> Self {
		ResultHeader {
			block_hash: None,
			extrinsics_root: t.extrinsics_root,
			parent_hash: t.parent_hash,
			state_root: t.state_root,
			number: t.number,
		}
	}
}

#[derive(Serialize, Debug)]
pub struct ResultBlock {
	pub header: ResultHeader,
	pub extrinsics: Vec<ResultTransaction>,
}

impl TryFrom<Block> for ResultBlock {
	type Error = errors::Error;

	fn try_from(t: Block) -> Result<Self, Self::Error> {
		// extrinsics
		let extrinsics = t.extrinsics;
		let extrinsics = extrinsics
			.into_iter()
			.enumerate()
			.map(|(index, x)| -> errors::Result<ResultTransaction> {
				let mut x = hex_decode(&x)?;
				let mut length_prefix: Vec<u8> = Compact(x.len() as u32).encode();
				length_prefix.append(&mut x);
				let raw = length_prefix;
				let x: Transaction =
					Decode::decode(&mut &raw[..]).ok_or(errors::ErrorKind::ParseError)?;
				let mut x: ResultTransaction = x.into();

				let hash = blake2_256(&raw);
				x.hash = Some(Hex(hash.to_vec()));
				x.index = Some(index as u32);

				Ok(x)
			})
			.collect::<errors::Result<Vec<_>>>()?;

		// success

		Ok(ResultBlock {
			header: t.header.into(),
			extrinsics: extrinsics,
		})
	}
}

impl TryFrom<ResultBlock> for Value {
	type Error = errors::Error;

	fn try_from(x: ResultBlock) -> Result<Self, Self::Error> {
		let x = serde_json::to_vec(&x).map_err(|_| errors::ErrorKind::ParseError)?;
		let x = serde_json::from_slice(&x).map_err(|_| errors::ErrorKind::ParseError)?;
		Ok(x)
	}
}

#[derive(Serialize, Debug)]
pub struct ResultSignature {
	#[serde(with = "SerdeHex")]
	pub sender: Vec<u8>,
	#[serde(with = "SerdeHex")]
	pub signature: Vec<u8>,
	pub nonce: u64,
	pub era: ResultEra,
}

#[derive(Serialize, Debug)]
pub struct ResultTransaction {
	pub hash: Option<Hex<Vec<u8>>>,
	pub signature: Option<ResultSignature>,
	pub call: Call,
	pub index: Option<u32>,
	pub success: Option<bool>,
}

#[derive(Serialize, Debug)]
pub enum ResultEra {
	Immortal,
	Mortal(u64, u64),
}

impl From<Era> for ResultEra {
	fn from(t: Era) -> Self {
		match t {
			Era::Immortal => Self::Immortal,
			Era::Mortal(period, phase) => Self::Mortal(period, phase),
		}
	}
}

impl From<Transaction> for ResultTransaction {
	fn from(t: Transaction) -> Self {
		let signature = t
			.signature
			.map(|(address, sig, nonce, era)| ResultSignature {
				sender: address.0.to_vec(),
				signature: sig.to_vec(),
				nonce: nonce.0,
				era: era.into(),
			});
		Self {
			hash: None,
			signature,
			call: t.call,
			index: None,
			success: None,
		}
	}
}

pub fn get_value_storage_key(key: &[u8]) -> StorageKey {
	StorageKey(twox_128(key))
}

pub fn get_map_storage_key(key: &[u8], prefix: &[u8]) -> StorageKey {
	let mut prefix = prefix.to_vec();
	prefix.extend(key);
	let a = blake2_256(&prefix).to_vec();
	StorageKey(a)
}

fn twox_128(data: &[u8]) -> Vec<u8> {
	let hash0 = twox(data, 0);
	let hash1 = twox(data, 1);
	let mut result = vec![0u8; 16];
	result[0..8].copy_from_slice(&hash0);
	result[8..16].copy_from_slice(&hash1);
	result
}

fn twox(data: &[u8], seed: u64) -> Vec<u8> {
	use ::core::hash::Hasher;
	let mut h = twox_hash::XxHash::with_seed(seed);
	h.write(&data);
	let r = h.finish();
	use byteorder::{ByteOrder, LittleEndian};
	let mut dest = vec![0u8; 8];
	LittleEndian::write_u64(&mut dest[0..8], r);
	dest
}

fn hex_decode(str: &str) -> errors::Result<Vec<u8>> {
	let str = str.trim_start_matches("0x");
	let result = hex::decode(str).map_err(|_| errors::ErrorKind::ParseError)?;
	Ok(result)
}
