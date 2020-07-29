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

use serde::{Deserialize, Serialize};

use crate::rpc::serde::SerdeHex;

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
pub struct ResultHeader {

    #[serde(with = "SerdeHex")]
    pub block_hash: Vec<u8>,

    #[serde(with = "SerdeHex")]
    pub extrinsics_root: Vec<u8>,

    #[serde(with = "SerdeHex")]
    pub parent_hash: Vec<u8>,

    #[serde(with = "SerdeHex")]
    pub state_root: Vec<u8>,

    pub number: BlockNumber,
}

impl ResultHeader {
    pub fn new(header: Header, block_hash: Vec<u8>) -> Self {
        Self{
            block_hash,
            extrinsics_root: header.extrinsics_root,
            parent_hash: header.parent_hash,
            state_root: header.state_root,
            number: header.number
        }
    }
}
