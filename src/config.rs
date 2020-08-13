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

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use app_dirs::{AppDataType, AppInfo};
use log::info;
use log::trace;
use mut_static::MutStatic;
use parity_codec::Decode;
use runtime_primitives::generic::DigestItem;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use tokio::runtime::Runtime;
use yee_primitives::Hrp;
use yee_runtime::{AuthorityId, Hash};
use yee_sharding::ShardingDigestItem;

use lazy_static::lazy_static;

use crate::errors;
use crate::opt::Opt;
use crate::rpc::client::RpcClient;

lazy_static! {
	pub static ref HRP: MutStatic<Hrp> = MutStatic::new();
	pub static ref SHARD_COUNT: MutStatic<u16> = MutStatic::new();
}

/// Config
/// # Configure file description
/// ### Path
/// <base_path>/conf/config.toml
///
/// ### Content
/// ```
/// [shards]
/// [shards.0]
/// rpc = ["http://127.0.0.1:9033"]
///
/// [shards.1]
/// rpc = ["http://127.0.0.1:9133"]
///
/// [shards.2]
/// rpc = ["http://127.0.0.1:9233"]
///
/// [shards.3]
/// rpc = ["http://127.0.0.1:9333"]
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shard {
	pub rpc: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
	pub shards: HashMap<String, Shard>,
}

pub struct VersionInfo {
	pub version: &'static str,
	pub executable_name: &'static str,
	pub author: &'static str,
}

pub fn get_config(opt: &Opt, version: &VersionInfo) -> errors::Result<Config> {
	let conf_path = conf_path(&base_path(opt, version));

	let conf_path = conf_path.join("config.toml");

	trace!("conf_path:{}", conf_path.to_string_lossy());

	let mut file =
		File::open(&conf_path).map_err(|_e| format!("Non-existed conf file: {:?}", conf_path))?;

	let mut str_val = String::new();
	file.read_to_string(&mut str_val)?;

	let conf: Config = toml::from_str(&str_val).map_err(|_e| "Error reading conf file")?;

	init_global(&conf)?;

	Ok(conf)
}

fn init_global(config: &Config) -> errors::Result<()> {
	init_hrp(config)?;

	init_shard_count(config)?;

	Ok(())
}

fn init_hrp(config: &Config) -> errors::Result<()> {
	let rpc_client = Arc::new(RpcClient::new(config.clone()));
	let future = rpc_client
		.call_method_async("system_chain", "", (), 0)
		.unwrap_or_else(|e| Box::new(futures::future::err(e.into())));

	let mut runtime = Runtime::new().expect("Failed to start new Runtime");
	let result: jsonrpc_core::Result<String> = runtime.block_on(future);

	let result = result.map_err(|_| "Failed to set shard_count")?;

	let hrp = match result.as_str() {
		"MainNet" => Hrp::MAINNET,
		_ => Hrp::TESTNET,
	};
	info!("set up hrp: {:?}", hrp);
	HRP.set(hrp).map_err(|_| "Failed to set hrp")?;

	Ok(())
}

fn init_shard_count(config: &Config) -> errors::Result<()> {
	let rpc_client = Arc::new(RpcClient::new(config.clone()));
	let future = rpc_client
		.call_method_async("chain_getHeader", "", (), 0)
		.unwrap_or_else(|e| Box::new(futures::future::err(e.into())));

	let mut runtime = Runtime::new().expect("Failed to start new Runtime");
	let result: jsonrpc_core::Result<Option<Value>> = runtime.block_on(future);

	let result = result.map_err(|_| "Failed to set shard_count")?;
	let result = result.ok_or("Failed to set shard_count".to_string())?;

	let digest = result
		.as_object()
		.ok_or("Failed to set shard_count".to_string())?
		.get("digest")
		.ok_or("Failed to set shard_count".to_string())?;

	let logs = get_logs(digest)?;

	let shard_info: Option<(u16, u16)> = logs
		.iter()
		.filter_map(ShardingDigestItem::as_sharding_info)
		.next();

	let (_, shard_count) = shard_info.ok_or("Failed to set shard_count".to_string())?;

	info!("set up shard_count: {}", shard_count);
	SHARD_COUNT
		.set(shard_count)
		.map_err(|_| "Failed to set shard_count")?;

	Ok(())
}

fn get_logs(digest: &Value) -> Result<Vec<DigestItem<Hash, AuthorityId, ()>>, String> {
	let logs = digest
		.as_object()
		.ok_or("none error".to_string())?
		.get("logs")
		.ok_or("none error".to_string())?
		.as_array()
		.ok_or("none error".to_string())?
		.iter()
		.filter_map(|x| {
			let x = x.as_str();
			match x {
				Some(x) => {
					let x = x.trim_start_matches("0x");
					match hex::decode(x) {
						Ok(x) => {
							let x: Option<DigestItem<Hash, AuthorityId, ()>> =
								Decode::decode(&mut &x[..]);
							x
						}
						Err(_) => None,
					}
				}
				None => None,
			}
		})
		.collect::<Vec<_>>();
	Ok(logs)
}

fn conf_path(base_path: &Path) -> PathBuf {
	let mut path = base_path.to_owned();
	path.push("conf");
	path
}

fn base_path(cli: &Opt, version: &VersionInfo) -> PathBuf {
	cli.base_path.clone().unwrap_or_else(|| {
		app_dirs::get_app_root(
			AppDataType::UserData,
			&AppInfo {
				name: version.executable_name,
				author: version.author,
			},
		)
		.expect("app directories exist on all supported platforms; qed")
	})
}
