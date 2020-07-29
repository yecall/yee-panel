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

use jsonrpc_core::BoxFuture;

use jsonrpc_derive::rpc;

use crate::config::Config;
use crate::rpc::client::RpcClient;

#[rpc]
pub trait ChainApi {
    #[rpc(name = "chain_getHeaderByHash")]
    fn get_header_by_hash(&self, shard_num: u16) -> BoxFuture<Option<u32>>;
}

pub struct Chain {
    config: Config,
    rpc_client: RpcClient,
}

impl Chain {
    /// Create new State API RPC handler.
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            rpc_client: RpcClient::new(config),
        }
    }
}

impl ChainApi for Chain {
    fn get_header_by_hash(&self, shard_num: u16) -> BoxFuture<Option<u32>> {
        Box::new(futures::future::ok(Some(10)))
    }
}
