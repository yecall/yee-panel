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

use crate::config::Config;
use crate::rpc::client::{self, RpcClient};
use crate::rpc::errors;
use crate::rpc::serde::Hex;
use crate::rpc::types::{get_value_storage_key, BlockNumber, ResultBlock, ResultHeader};

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

	#[rpc(name = "chain_getBlockByNumber")]
	fn get_block_by_number(&self, shard_num: u16, number: BlockNumber) -> BoxFuture<Option<Value>>;
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

	fn get_block_by_number(&self, shard_num: u16, number: BlockNumber) -> BoxFuture<Option<Value>> {
		match check_shard_num(shard_num, &self.config) {
			Err(e) => return Box::new(future::err(e.into())),
			_ => (),
		}

		// get block hash
		let get_block_hash = || -> Box<dyn Future<Item=jsonrpc_core::Result<Option<Hex<Vec<u8>>>>, Error=jsonrpc_core::Error> + Send> {
            let result = client::get_block_hash_future(self.rpc_client.clone(), number, shard_num);
            let result = result.map(|x| Ok(x));
            Box::new(result)
        };
		let result = get_block_hash();

		// get block
		let rpc_client = self.rpc_client.clone();
		let get_block = move || -> Box<dyn Future<Item=jsonrpc_core::Result<Option<ResultBlock>>, Error=jsonrpc_core::Error> + Send> {
            let result = result.and_then(move |x| {
                match x {
                    Ok(Some(hash)) => {
                        let result = client::get_block_future(rpc_client, &Some(hash.clone()), shard_num);
                        let result = result.map(|x| -> jsonrpc_core::Result<Option<ResultBlock>> {
                            let block = match x {
                                Some(block) => block,
                                None => return Ok(None),
                            };
                            let mut block : ResultBlock = block.block.try_into()?;
                            block.header.block_hash = Some(hash);
                            Ok(Some(block))
                        });
                        Box::new(result) as Box<dyn Future<Item=jsonrpc_core::Result<Option<ResultBlock>>, Error=jsonrpc_core::Error> + Send>
                    },
                    Ok(None) => Box::new(future::ok(Ok(None))),
                    Err(e) => Box::new(future::err(e)),
                }
            });
            Box::new(result)
        };
		let result = get_block();

		// get block with extrinsic result
		let rpc_client = self.rpc_client.clone();
		let get_block_with_extrinsic_result = move || -> Box<dyn Future<Item=jsonrpc_core::Result<Option<ResultBlock>>, Error=jsonrpc_core::Error> + Send> {

            let result = result.and_then(move |x| {
                match x {
                    Ok(Some(mut block)) => {

                        let events_storage_key = get_value_storage_key(b"System Events");
                        let events_storage_key = &Hex(events_storage_key.0);
                        let block_hash = &Some(block.header.block_hash.as_ref().expect("qed").clone());
                        let events = client::get_storage_future(rpc_client, events_storage_key, block_hash, shard_num);
                        let result = events.map(|x| -> jsonrpc_core::Result<Option<ResultBlock>> {
                            let result = get_block_extrinsics_result(x)?;
                            for (index, tx) in &mut block.extrinsics.iter_mut().enumerate() {
                                tx.success = result.get(&(index as u32)).as_ref().map(|x| x.0);
                            }
                            Ok(Some(block))
                        });
                        Box::new(result) as Box<dyn Future<Item=jsonrpc_core::Result<Option<ResultBlock>>, Error=jsonrpc_core::Error> + Send>
                    },
                    Ok(None) => Box::new(future::ok(Ok(None))),
                    Err(e) => Box::new(future::err(e)),
                }
            });
            Box::new(result)
        };
		let result = get_block_with_extrinsic_result();

		// convert to value to avoid jsonrpc u128 serialize problem
		let convert_to_value = || -> Box<dyn Future<Item=jsonrpc_core::Result<Option<Value>>, Error=jsonrpc_core::Error> + Send> {
            let result = result.map(|x| {
                match x {
                    Ok(Some(block)) => {
                        let result: Value = block.try_into()?;
                        Ok(Some(result))
                    },
                    Ok(None) => Ok(None),
                    Err(e) => Err(e),
                }
            });
            Box::new(result)
        };
		let result = convert_to_value();

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
