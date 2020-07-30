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

use std::sync::Arc;

use futures::future;
use futures::future::Future;
use jsonrpc_core::BoxFuture;
use jsonrpc_derive::rpc;

use crate::config::Config;
use crate::rpc::client::RpcClient;
use crate::rpc::errors;
use crate::rpc::serde::Hex;
use crate::rpc::types::{BlockNumber, Header, ResultHeader, ResultBlock, Block};

#[rpc]
pub trait ChainApi {
    #[rpc(name = "chain_getBestNumber")]
    fn get_best_number(&self, shard_num: u16) -> BoxFuture<Option<BlockNumber>>;

    #[rpc(name = "chain_getFinalizedNumber")]
    fn get_finalized_number(&self, shard_num: u16) -> BoxFuture<Option<BlockNumber>>;

    #[rpc(name = "chain_getHeaderByNumber")]
    fn get_header_by_number(&self, shard_num: u16, number: BlockNumber) -> BoxFuture<Option<ResultHeader>>;

    #[rpc(name = "chain_getBlockByNumber")]
    fn get_block_by_number(&self, shard_num: u16, number: BlockNumber) -> BoxFuture<Option<ResultBlock>>;
}

pub struct Chain {
    config: Config,
    rpc_client: Arc<RpcClient>,
}

impl Chain {
    /// Create new State API RPC handler.
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            rpc_client: Arc::new(RpcClient::new(config)),
        }
    }

    fn get_block_hash_future(rpc_client: Arc<RpcClient>, number: BlockNumber, shard_num: u16) -> Box<dyn Future<Item=Option<Hex<Vec<u8>>>, Error=jsonrpc_core::Error> + Send> {
        let result : BoxFuture<Option<Hex<Vec<u8>>>> =  rpc_client.call_method_async("chain_getBlockHash", "", (number, ), shard_num)
            .unwrap_or_else(|e|Box::new(future::err(e.into())));
        Box::new(result)
    }

    fn get_finalized_hash_future(rpc_client: Arc<RpcClient>, shard_num: u16) -> Box<dyn Future<Item=Option<Hex<Vec<u8>>>, Error=jsonrpc_core::Error> + Send> {
        let result : BoxFuture<Option<Hex<Vec<u8>>>> =  rpc_client.call_method_async("chain_getFinalizedHead", "", (), shard_num)
            .unwrap_or_else(|e|Box::new(future::err(e.into())));
        Box::new(result)
    }

    fn get_header_future(rpc_client: Arc<RpcClient>, hash: &Option<Hex<Vec<u8>>>, shard_num: u16) -> Box<dyn Future<Item=Option<Header>, Error=jsonrpc_core::Error> + Send> {

        let result: BoxFuture<Option<Header>>  = match hash{
            Some(hash) => {
                let params = (hash.to_string(), );
                rpc_client.call_method_async("chain_getHeader", "", params, shard_num)
                    .unwrap_or_else(|e|Box::new(future::err(e.into())))
            },
            None => {
                let params = ();
                rpc_client.call_method_async("chain_getHeader", "", params, shard_num)
                    .unwrap_or_else(|e|Box::new(future::err(e.into())))
            },
        };
        Box::new(result)
    }

    fn get_block_future(rpc_client: Arc<RpcClient>, hash: &Option<Hex<Vec<u8>>>, shard_num: u16) -> Box<dyn Future<Item=Option<Block>, Error=jsonrpc_core::Error> + Send> {

        let result: BoxFuture<Option<Header>>  = match hash{
            Some(hash) => {
                let params = (hash.to_string(), );
                rpc_client.call_method_async("chain_getBlock", "", params, shard_num)
                    .unwrap_or_else(|e|Box::new(future::err(e.into())))
            },
            None => {
                let params = ();
                rpc_client.call_method_async("chain_getBlock", "", params, shard_num)
                    .unwrap_or_else(|e|Box::new(future::err(e.into())))
            },
        };
        Box::new(result)
    }
}

impl ChainApi for Chain {
    fn get_best_number(&self, shard_num: u16) -> BoxFuture<Option<BlockNumber>> {

        match check_shard_num(shard_num, &self.config){
            Err(e) => return Box::new(future::err(e.into())),
            _ => (),
        }

        let result = Self::get_header_future( self.rpc_client.clone(), &None, shard_num);

        let result = result.map(|x|{
            x.map(|x|x.number)
        });

        Box::new(result)
    }

    fn get_finalized_number(&self, shard_num: u16) -> BoxFuture<Option<BlockNumber>> {

        match check_shard_num(shard_num, &self.config){
            Err(e) => return Box::new(future::err(e.into())),
            _ => (),
        }

        let result  = Self::get_finalized_hash_future(self.rpc_client.clone(),  shard_num);

        let rpc_client = self.rpc_client.clone();
        let result = result.and_then(move |hash| {
            let header = Self::get_header_future(rpc_client, &hash, shard_num);
            header
        });

        let result = result.map(|x| {
            x.map(|x|x.number)
        });

        Box::new(result)
    }

    fn get_header_by_number(&self, shard_num: u16, number: BlockNumber) -> BoxFuture<Option<ResultHeader>> {

        match check_shard_num(shard_num, &self.config){
            Err(e) => return Box::new(future::err(e.into())),
            _ => (),
        }

        let result = Self::get_block_hash_future(self.rpc_client.clone(), number, shard_num);

        let rpc_client = self.rpc_client.clone();
        let result = result.and_then(move |hash| {
            let header = Self::get_header_future(rpc_client, &hash, shard_num);
            header.map(|header| {
                match (header, hash) {
                    (Some(header), Some(hash)) => Some(ResultHeader::new(header, hash.0)),
                    _ => None,
                }
            })
        });

        Box::new(result)
    }

    fn get_block_by_number(&self, shard_num: u16, number: BlockNumber) -> BoxFuture<Option<ResultBlock>> {

        match check_shard_num(shard_num, &self.config){
            Err(e) => return Box::new(future::err(e.into())),
            _ => (),
        }

        let result = Self::get_block_hash_future(self.rpc_client.clone(), number, shard_num);

        let rpc_client = self.rpc_client.clone();
        let result = result.and_then(move |hash| {
            let header = Self::get_block_future(rpc_client, &hash, shard_num);
            header.map(|block| {
                match (block, hash) {
                    (Some(block), Some(hash)) => Some(ResultBlock::new(block, hash.0)),
                    _ => None,
                }
            })
        });

        Box::new(result)
    }
}

fn check_shard_num(shard_num: u16, config: &Config) -> errors::Result<()> {
    if shard_num > config.shards.len() as u16 {
        return Err(errors::ErrorKind::InvalidShard.into())
    }
    Ok(())
}
