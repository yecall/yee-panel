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

pub type Hash = primitive_types::H256;

pub type BlockNumber = u64;

pub type Nonce = u64;

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    #[serde(with = "SerdeHex")]
    pub extrinsicsRoot: Vec<u8>,

    #[serde(with = "SerdeHex")]
    pub number: BlockNumber,
}