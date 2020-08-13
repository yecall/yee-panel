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

use std::net::SocketAddr;

use futures::future::Future;
use log::info;

use crate::config::Config;
use crate::opt::{Opt, DEFAULT_RPC_PORT, DEFAULT_WS_PORT};
use crate::rpc::chain::{Chain, ChainApi};
use crate::rpc::server::{start_http, start_ws};

mod chain;
pub mod client;
pub mod errors;
mod metadata;
mod serde;
mod server;
mod types;

pub fn run(opt: &Opt, config: &Config) -> errors::Result<()> {
	let rpc_interface: &str = if opt.rpc_external {
		"0.0.0.0"
	} else {
		"127.0.0.1"
	};
	let ws_interface: &str = if opt.ws_external {
		"0.0.0.0"
	} else {
		"127.0.0.1"
	};

	let rpc_address_http = parse_address(
		&format!("{}:{}", rpc_interface, DEFAULT_RPC_PORT),
		opt.rpc_port,
	)?;
	let rpc_address_ws = parse_address(
		&format!("{}:{}", ws_interface, DEFAULT_WS_PORT),
		opt.ws_port,
	)?;

	let (signal, exit) = exit_future::signal();

	let handler = || {
		let chain = Chain::new(config.clone());

		let mut io = pubsub::PubSubHandler::default();
		io.extend_with(chain.to_delegate());
		io
	};

	let _server = start_http(&rpc_address_http, handler())?;

	info!("Switch rpc http listen on: {}", rpc_address_http);

	let _server = start_ws(&rpc_address_ws, handler())?;

	info!("Switch rpc ws listen on: {}", rpc_address_ws);

	exit.wait().unwrap();

	signal.fire();

	Ok(())
}

fn parse_address(address: &str, port: Option<u16>) -> errors::Result<SocketAddr> {
	let mut address: SocketAddr = address
		.parse()
		.map_err(|_| format!("Invalid address: {}", address))?;
	if let Some(port) = port {
		address.set_port(port);
	}

	Ok(address)
}
