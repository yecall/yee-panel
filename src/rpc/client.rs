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
use futures::Future;
use jsonrpc_client_transports::RpcError;
use jsonrpc_core::BoxFuture;
use jsonrpc_core_client::TypedClient;
use rand::Rng;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::config::Config;
use crate::rpc::errors;
use crate::rpc::serde::Hex;
use crate::rpc::types::{BlockNumber, BlockResponse, Header};

pub struct RpcClient {
    config: Config,
}

impl RpcClient {
    pub fn new(config: Config) -> Self {
        RpcClient { config }
    }

    fn get_random_rpc_uri(&self, shard_num: u16) -> errors::Result<String> {
        let shard = self
            .config
            .shards
            .get(&format!("{}", shard_num))
            .ok_or(errors::Error::from(errors::ErrorKind::ConfigError))?;

        let rpc = &shard.rpc;

        if rpc.len() == 0 {
            return Err(errors::Error::from(errors::ErrorKind::ConfigError));
        }

        let mut rng = rand::thread_rng();

        let i = rng.gen_range(0, rpc.len());

        Ok(rpc[i].clone())
    }

    #[allow(dead_code)]
    pub fn call_method<T: Serialize, R: DeserializeOwned + 'static>(
        &self,
        method: &str,
        returns: &'static str,
        args: T,
        shard_num: u16,
    ) -> errors::Result<R> {
        let uri = self.get_random_rpc_uri(shard_num)?;

        let result = jsonrpc_core_client::transports::http::connect(&uri)
            .and_then(|client: TypedClient| {
                client
                    .call_method(method, returns, args)
                    .and_then(move |result| Ok(result))
            })
            .wait()
            .map_err(|e| {
                log::error!("RPC Client error: {:?}", e);
                e
            })
            .map_err(parse_error);

        result
    }

    pub fn call_method_async<
        T: Serialize + 'static + Send,
        R: DeserializeOwned + 'static + Send,
    >(
        &self,
        method: &str,
        _returns: &'static str,
        args: T,
        shard_num: u16,
    ) -> errors::Result<BoxFuture<R>> {
        let uri = self.get_random_rpc_uri(shard_num)?;

        let method = method.to_owned();

        let run = jsonrpc_core_client::transports::http::connect(&uri)
            .and_then(move |client: TypedClient| {
                client
                    .call_method(&method, "returns", args)
                    .and_then(move |result| Ok(result))
            })
            .map_err(|e| {
                log::error!("RPC Client error: {:?}", e);
                e
            })
            .map_err(parse_error)
            .map_err(|e| e.into());

        Ok(Box::new(run))
    }
}

pub fn get_block_hash_future(
    rpc_client: Arc<RpcClient>,
    number: BlockNumber,
    shard_num: u16,
) -> Box<dyn Future<Item=Option<Hex<Vec<u8>>>, Error=jsonrpc_core::Error> + Send> {
    let result: BoxFuture<Option<Hex<Vec<u8>>>> = rpc_client
        .call_method_async("chain_getBlockHash", "", (number, ), shard_num)
        .unwrap_or_else(|e| Box::new(future::err(e.into())));
    Box::new(result)
}

pub fn get_finalized_hash_future(
    rpc_client: Arc<RpcClient>,
    shard_num: u16,
) -> Box<dyn Future<Item=Option<Hex<Vec<u8>>>, Error=jsonrpc_core::Error> + Send> {
    let result: BoxFuture<Option<Hex<Vec<u8>>>> = rpc_client
        .call_method_async("chain_getFinalizedHead", "", (), shard_num)
        .unwrap_or_else(|e| Box::new(future::err(e.into())));
    Box::new(result)
}

pub fn get_header_future(
    rpc_client: Arc<RpcClient>,
    hash: &Option<Hex<Vec<u8>>>,
    shard_num: u16,
) -> Box<dyn Future<Item=Option<Header>, Error=jsonrpc_core::Error> + Send> {
    let result: BoxFuture<Option<Header>> = match hash {
        Some(hash) => {
            let params = (hash.to_string(), );
            rpc_client
                .call_method_async("chain_getHeader", "", params, shard_num)
                .unwrap_or_else(|e| Box::new(future::err(e.into())))
        }
        None => {
            let params = ();
            rpc_client
                .call_method_async("chain_getHeader", "", params, shard_num)
                .unwrap_or_else(|e| Box::new(future::err(e.into())))
        }
    };
    Box::new(result)
}

pub fn get_block_future(
    rpc_client: Arc<RpcClient>,
    hash: &Option<Hex<Vec<u8>>>,
    shard_num: u16,
) -> Box<dyn Future<Item=Option<BlockResponse>, Error=jsonrpc_core::Error> + Send> {
    let result: BoxFuture<Option<BlockResponse>> = match hash {
        Some(hash) => {
            let params = (hash.to_string(), );
            rpc_client
                .call_method_async("chain_getBlock", "", params, shard_num)
                .unwrap_or_else(|e| Box::new(future::err(e.into())))
        }
        None => {
            let params = ();
            rpc_client
                .call_method_async("chain_getBlock", "", params, shard_num)
                .unwrap_or_else(|e| Box::new(future::err(e.into())))
        }
    };
    Box::new(result)
}

pub fn get_storage_future(
    rpc_client: Arc<RpcClient>,
    key: &Hex<Vec<u8>>,
    block_hash: &Option<Hex<Vec<u8>>>,
    shard_num: u16,
) -> Box<dyn Future<Item=Option<Hex<Vec<u8>>>, Error=jsonrpc_core::Error> + Send> {
    let key = key.to_string();
    let block_hash = block_hash.as_ref().map(|x| x.to_string());

    let result: BoxFuture<Option<Hex<Vec<u8>>>> = rpc_client
        .call_method_async("state_getStorage", "", (key, block_hash), shard_num)
        .unwrap_or_else(|e| Box::new(future::err(e.into())));
    Box::new(result)
}

pub fn submit_extrinsic_future(
    rpc_client: Arc<RpcClient>,
    raw: &Hex<Vec<u8>>,
    shard_num: u16,
) -> Box<dyn Future<Item=Hex<Vec<u8>>, Error=jsonrpc_core::Error> + Send> {

    let result: BoxFuture<Option<Hex<Vec<u8>>>> = rpc_client
        .call_method_async("author_submitExtrinsic", "", (raw,), shard_num)
        .unwrap_or_else(|e| Box::new(future::err(e.into())));
    Box::new(result)
}

fn parse_error(error: RpcError) -> errors::Error {
    errors::Error::from(errors::ErrorKind::RpcError(error))
}
