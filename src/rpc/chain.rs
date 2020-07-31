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

use std::collections::{hash_map::Entry, HashMap};
use std::convert::TryInto;
use std::sync::Arc;

use futures::future;
use futures::future::Future;
use jsonrpc_core::BoxFuture;
use jsonrpc_derive::rpc;
use parity_codec::Decode;
use serde_json::Value;
use srml_system::{EventRecord, Phase};
use yee_runtime::Event;
use yee_primitives::Address;
use yee_primitives::AddressCodec;
use yee_sharding_primitives::utils::shard_num_for_bytes;

use crate::config::Config;
use crate::rpc::client::{self, RpcClient};
use crate::rpc::errors;
use crate::rpc::serde::Hex;
use crate::rpc::types::{get_value_storage_key, BlockNumber, ResultBlock, ResultHeader, ResultTransaction, Nonce, get_map_storage_key};

#[rpc]
pub trait ChainApi {
    #[rpc(name = "chain_getBestNumber")]
    fn get_best_number(&self, shard_num: u16) -> BoxFuture<Option<BlockNumber>>;

    #[rpc(name = "chain_getFinalizedNumber")]
    fn get_finalized_number(&self, shard_num: u16) -> BoxFuture<Option<BlockNumber>>;

    #[rpc(name = "chain_getHeaderByNumber")]
    fn get_header_by_number(
        &self,
        shard_num: u16,
        number: BlockNumber,
    ) -> BoxFuture<Option<ResultHeader>>;

    #[rpc(name = "chain_getHeaderByHash")]
    fn get_header_by_hash(
        &self,
        shard_num: u16,
        hash: Hex<Vec<u8>>,
    ) -> BoxFuture<Option<ResultHeader>>;

    #[rpc(name = "chain_getBlockByNumber")]
    fn get_block_by_number(&self, shard_num: u16, number: BlockNumber) -> BoxFuture<Option<Value>>;

    #[rpc(name = "chain_getBlockByHash")]
    fn get_block_by_hash(&self, shard_num: u16, hash: Hex<Vec<u8>>) -> BoxFuture<Option<Value>>;

    #[rpc(name = "chain_getExtrinsicByHash")]
    fn get_extrinsic_by_hash(&self, shard_num: u16, block_number: BlockNumber, hash: Hex<Vec<u8>>) -> BoxFuture<Option<Value>>;

    #[rpc(name = "chain_getExtrinsicByRaw")]
    fn get_extrinsic_by_raw(&self, shard_num: u16, block_number: BlockNumber, raw: Hex<Vec<u8>>) -> BoxFuture<Option<Value>>;

    #[rpc(name = "state_getNonce")]
    fn get_nonce(&self, address: String, block_number: Option<BlockNumber>) -> BoxFuture<Nonce>;
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
}

impl ChainApi for Chain {
    fn get_best_number(&self, shard_num: u16) -> BoxFuture<Option<BlockNumber>> {
        match check_shard_num(shard_num, &self.config) {
            Err(e) => return Box::new(future::err(e.into())),
            _ => (),
        }

        let result = client::get_header_future(self.rpc_client.clone(), &None, shard_num);

        let result = result.map(|x| x.map(|x| x.number));

        Box::new(result)
    }

    fn get_finalized_number(&self, shard_num: u16) -> BoxFuture<Option<BlockNumber>> {
        match check_shard_num(shard_num, &self.config) {
            Err(e) => return Box::new(future::err(e.into())),
            _ => (),
        }

        let result = client::get_finalized_hash_future(self.rpc_client.clone(), shard_num);

        let rpc_client = self.rpc_client.clone();
        let result = result.and_then(move |hash| {
            let header = client::get_header_future(rpc_client, &hash, shard_num);
            header
        });

        let result = result.map(|x| x.map(|x| x.number));

        Box::new(result)
    }

    fn get_header_by_number(
        &self,
        shard_num: u16,
        number: BlockNumber,
    ) -> BoxFuture<Option<ResultHeader>> {
        match check_shard_num(shard_num, &self.config) {
            Err(e) => return Box::new(future::err(e.into())),
            _ => (),
        }

        let result = client::get_block_hash_future(self.rpc_client.clone(), number, shard_num);

        let rpc_client = self.rpc_client.clone();
        let result = result.and_then(move |hash| {
            let header = client::get_header_future(rpc_client, &hash, shard_num);
            header.map(|header| match (header, hash) {
                (Some(header), Some(hash)) => {
                    let mut header: ResultHeader = header.into();
                    header.block_hash = Some(hash);
                    Some(header)
                }
                _ => None,
            })
        });

        Box::new(result)
    }

    fn get_header_by_hash(
        &self,
        shard_num: u16,
        hash: Hex<Vec<u8>>,
    ) -> BoxFuture<Option<ResultHeader>> {
        match check_shard_num(shard_num, &self.config) {
            Err(e) => return Box::new(future::err(e.into())),
            _ => (),
        }

        let result = future::ok(Some(hash));

        let rpc_client = self.rpc_client.clone();
        let result = result.and_then(move |hash| {
            let header = client::get_header_future(rpc_client, &hash, shard_num);
            header.map(|header| match (header, hash) {
                (Some(header), Some(hash)) => {
                    let mut header: ResultHeader = header.into();
                    header.block_hash = Some(hash);
                    Some(header)
                }
                _ => None,
            })
        });

        Box::new(result)
    }

    fn get_block_by_number(&self, shard_num: u16, number: BlockNumber) -> BoxFuture<Option<Value>> {
        match check_shard_num(shard_num, &self.config) {
            Err(e) => return Box::new(future::err(e.into())),
            _ => (),
        }

        // get block hash
        let get_block_hash = || -> BoxFuture<jsonrpc_core::Result<Option<Hex<Vec<u8>>>>> {
            let result = client::get_block_hash_future(self.rpc_client.clone(), number, shard_num);
            let result = result.map(|x| Ok(x));
            Box::new(result)
        };
        let result = get_block_hash();

        let result = get_block_future(self.rpc_client.clone(), shard_num, false, result);

        let result = get_value_future(result);

        let result = result.and_then(|x| match x {
            Ok(v) => future::ok(v),
            Err(e) => future::err(e),
        });


        Box::new(result)
    }

    fn get_block_by_hash(&self, shard_num: u16, hash: Hex<Vec<u8>>) -> BoxFuture<Option<Value>> {
        match check_shard_num(shard_num, &self.config) {
            Err(e) => return Box::new(future::err(e.into())),
            _ => (),
        }

        let result = Box::new(future::ok(Ok(Some(hash))));

        let result = get_block_future(self.rpc_client.clone(), shard_num, false, result);

        let result = get_value_future(result);

        let result = result.and_then(|x| match x {
            Ok(v) => future::ok(v),
            Err(e) => future::err(e),
        });

        Box::new(result)
    }

    fn get_extrinsic_by_hash(&self, shard_num: u16, block_number: BlockNumber, hash: Hex<Vec<u8>>) -> BoxFuture<Option<Value>> {
        match check_shard_num(shard_num, &self.config) {
            Err(e) => return Box::new(future::err(e.into())),
            _ => (),
        }

        // get block hash
        let get_block_hash = || -> BoxFuture<jsonrpc_core::Result<Option<Hex<Vec<u8>>>>> {
            let result = client::get_block_hash_future(self.rpc_client.clone(), block_number, shard_num);
            let result = result.map(|x| Ok(x));
            Box::new(result)
        };
        let result = get_block_hash();

        let result = get_block_future(self.rpc_client.clone(), shard_num, false, result);

        // filter
        let filter = move || -> BoxFuture<jsonrpc_core::Result<Option<ResultTransaction>>> {
            let result = result.map(move |x| {
                match x {
                    Ok(Some(block)) => {
                        let extrinsic = block.extrinsics.into_iter().filter_map(|tx| {
                            if tx.hash.as_ref() == Some(&hash) { Some(tx) } else { None }
                        }).next();
                        Ok(extrinsic)
                    }
                    Ok(None) => Ok(None),
                    Err(e) => Err(e),
                }
            });
            Box::new(result)
        };
        let result = filter();

        let result = get_value_future(result);

        let result = result.and_then(|x| match x {
            Ok(v) => future::ok(v),
            Err(e) => future::err(e),
        });

        Box::new(result)
    }

    fn get_extrinsic_by_raw(&self, shard_num: u16, block_number: BlockNumber, raw: Hex<Vec<u8>>) -> BoxFuture<Option<Value>> {
        match check_shard_num(shard_num, &self.config) {
            Err(e) => return Box::new(future::err(e.into())),
            _ => (),
        }

        // get block hash
        let get_block_hash = || -> BoxFuture<jsonrpc_core::Result<Option<Hex<Vec<u8>>>>> {
            let result = client::get_block_hash_future(self.rpc_client.clone(), block_number, shard_num);
            let result = result.map(|x| Ok(x));
            Box::new(result)
        };
        let result = get_block_hash();

        let result = get_block_future(self.rpc_client.clone(), shard_num, true, result);

        // filter
        let filter = move || -> BoxFuture<jsonrpc_core::Result<Option<ResultTransaction>>> {
            let result = result.map(move |x| {
                match x {
                    Ok(Some(block)) => {
                        let extrinsic = block.extrinsics.into_iter().filter_map(|tx| {
                            if tx.raw.as_ref() == Some(&raw) { Some(tx) } else { None }
                        }).next();
                        let extrinsic = extrinsic.map(|mut x| {
                            x.raw = None;
                            x
                        });
                        Ok(extrinsic)
                    }
                    Ok(None) => Ok(None),
                    Err(e) => Err(e),
                }
            });
            Box::new(result)
        };
        let result = filter();

        let result = get_value_future(result);

        let result = result.and_then(|x| match x {
            Ok(v) => future::ok(v),
            Err(e) => future::err(e),
        });

        Box::new(result)
    }

    fn get_nonce(&self, address: String, block_number: Option<BlockNumber>) -> BoxFuture<Nonce> {
        let address = Address(address);

        let (public_key, _hrp) =
            match <[u8; 32]>::from_address(&address).map_err(|_| errors::Error::from(errors::ErrorKind::InvalidAddress).into()) {
                Ok(v) => v,
                Err(e) => return Box::new(future::err(e)),
            };

        let shard_count = self.config.shards.len() as u16;

        let shard_num = match shard_num_for_bytes(&public_key, shard_count) {
            Some(shard_num) => shard_num,
            None => return Box::new(future::err(errors::Error::from(errors::ErrorKind::InvalidShard).into())),
        };
        let storage_key = get_map_storage_key(&public_key, b"System AccountNonce");
        let storage_key = Hex(storage_key.0);

        // get block hash
        let get_block_hash = || -> BoxFuture<jsonrpc_core::Result<Option<Hex<Vec<u8>>>>> {
            match block_number {
                Some(block_number) => {
                    let result = client::get_block_hash_future(self.rpc_client.clone(), block_number, shard_num);
                    let result = result.map(|x| Ok(x));
                    Box::new(result)
                }
                None => Box::new(future::ok(Ok(None)))
            }
        };
        let result = get_block_hash();

        // get nonce
        let rpc_client = self.rpc_client.clone();
        let get_nonce = move || -> BoxFuture<jsonrpc_core::Result<Nonce>> {
            let result = result.and_then(move |x| {
                match x {
                    Ok(block_hash) => {
                        let result = client::get_storage_future(rpc_client, &storage_key, &block_hash, shard_num);
                        let result = result.map(|x| -> jsonrpc_core::Result<Nonce> {
                            match x {
                                Some(x) => {
                                    Ok(u64_from_slice(&x.0)?)
                                }
                                None => Ok(0),
                            }
                        });
                        Box::new(result) as BoxFuture<jsonrpc_core::Result<Nonce>>
                    }
                    Err(e) => Box::new(future::err(e)),
                }
            });
            Box::new(result)
        };
        let result = get_nonce();

        let result = result.and_then(|x| match x {
            Ok(v) => future::ok(v),
            Err(e) => future::err(e),
        });

        Box::new(result)
    }
}

fn check_shard_num(shard_num: u16, config: &Config) -> errors::Result<()> {
    if shard_num > config.shards.len() as u16 {
        return Err(errors::ErrorKind::InvalidShard.into());
    }
    Ok(())
}

fn get_block_extrinsics_result(
    events: Option<Hex<Vec<u8>>>,
) -> errors::Result<HashMap<u32, (bool, Vec<String>)>> {
    let mut result = HashMap::new();

    let events = match events {
        Some(events) => events,
        None => return Ok(result),
    };

    let events: Vec<EventRecord<Event>> =
        Decode::decode(&mut &events.0[..]).ok_or(errors::ErrorKind::ParseError)?;

    for event in events.into_iter() {
        match event.phase {
            Phase::ApplyExtrinsic(index) => match &event.event {
                Event::system(system_event) => {
                    let success = match system_event {
                        srml_system::Event::ExtrinsicSuccess => true,
                        srml_system::Event::ExtrinsicFailed => false,
                    };
                    match result.entry(index) {
                        Entry::Vacant(entry) => {
                            entry.insert((success, vec![]));
                        }
                        Entry::Occupied(mut entry) => {
                            let entry = entry.get_mut();
                            entry.0 = success;
                        }
                    }
                }
                _ => {
                    let event_str = format!("{:?}", event.event);
                    match result.entry(index) {
                        Entry::Vacant(entry) => {
                            entry.insert((false, vec![event_str]));
                        }
                        Entry::Occupied(mut entry) => {
                            let entry = entry.get_mut();
                            entry.1.push(event_str);
                        }
                    }
                }
            },
            _ => {}
        }
    }

    Ok(result)
}

fn get_block_future(rpc_client: Arc<RpcClient>, shard_num: u16, with_raw: bool, hash_future: BoxFuture<jsonrpc_core::Result<Option<Hex<Vec<u8>>>>>) -> BoxFuture<jsonrpc_core::Result<Option<ResultBlock>>> {

    // get block
    let tmp_rpc_client = rpc_client.clone();
    let get_block = move || -> BoxFuture<jsonrpc_core::Result<Option<ResultBlock>>> {
        let result = hash_future.and_then(move |x| {
            match x {
                Ok(Some(hash)) => {
                    let result = client::get_block_future(tmp_rpc_client, &Some(hash.clone()), shard_num);
                    let result = result.map(|x| -> jsonrpc_core::Result<Option<ResultBlock>> {
                        let block = match x {
                            Some(block) => block,
                            None => return Ok(None),
                        };
                        let mut block: ResultBlock = block.block.try_into()?;
                        block.header.block_hash = Some(hash);
                        Ok(Some(block))
                    });
                    Box::new(result) as BoxFuture<jsonrpc_core::Result<Option<ResultBlock>>>
                }
                Ok(None) => Box::new(future::ok(Ok(None))),
                Err(e) => Box::new(future::err(e)),
            }
        });
        Box::new(result)
    };
    let result = get_block();

    // get block with extrinsic result
    let tmp_rpc_client = rpc_client.clone();
    let get_block_with_extrinsic_result = move || -> BoxFuture<jsonrpc_core::Result<Option<ResultBlock>>> {
        let result = result.and_then(move |x| {
            match x {
                Ok(Some(mut block)) => {
                    let events_storage_key = get_value_storage_key(b"System Events");
                    let events_storage_key = &Hex(events_storage_key.0);
                    let block_hash = &Some(block.header.block_hash.as_ref().expect("qed").clone());
                    let events = client::get_storage_future(tmp_rpc_client, events_storage_key, block_hash, shard_num);
                    let result = events.map(move |x| -> jsonrpc_core::Result<Option<ResultBlock>> {
                        let result = get_block_extrinsics_result(x)?;
                        for (index, tx) in &mut block.extrinsics.iter_mut().enumerate() {
                            if !with_raw {
                                tx.raw = None;
                            }
                            tx.success = result.get(&(index as u32)).as_ref().map(|x| x.0);
                        }
                        Ok(Some(block))
                    });
                    Box::new(result) as BoxFuture<jsonrpc_core::Result<Option<ResultBlock>>>
                }
                Ok(None) => Box::new(future::ok(Ok(None))),
                Err(e) => Box::new(future::err(e)),
            }
        });
        Box::new(result)
    };
    let result = get_block_with_extrinsic_result();

    Box::new(result)
}

fn get_value_future<T>(future: BoxFuture<jsonrpc_core::Result<Option<T>>>) -> BoxFuture<jsonrpc_core::Result<Option<Value>>>
    where
        T: TryInto<Value> + 'static,
{

    // convert to value to avoid jsonrpc u128 serialize problem
    let convert_to_value = || -> BoxFuture<jsonrpc_core::Result<Option<Value>>> {
        let result = future.map(|x| {
            match x {
                Ok(Some(block)) => {
                    let result: Value = block.try_into().map_err(|_| errors::Error::from(errors::ErrorKind::ParseError))?;
                    Ok(Some(result))
                }
                Ok(None) => Ok(None),
                Err(e) => Err(e),
            }
        });
        Box::new(result)
    };
    let result = convert_to_value();

    Box::new(result)
}

fn u64_from_slice(bytes: &[u8]) -> errors::Result<u64> {
    const LEN: usize = 8;
    if bytes.len() != LEN {
        return Err(errors::ErrorKind::ParseError.into());
    }
    let mut array = [0; LEN];
    let bytes = &bytes[..];
    array.copy_from_slice(bytes);
    Ok(u64::from_le_bytes(array))
}
