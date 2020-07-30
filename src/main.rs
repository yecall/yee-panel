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

use crate::config::{get_config, VersionInfo};
use structopt::StructOpt;

mod config;
mod errors;
mod opt;
mod rpc;

fn main() {
	let result = run();
	match result {
		Ok(_) => (),
		Err(e) => eprintln!("{:?}", e),
	}
}

fn run() -> errors::Result<()> {
	let opt = opt::Opt::from_args();

	init_logger(&opt.log)?;

	let version_info = VersionInfo {
		version: env!("CARGO_PKG_VERSION"),
		executable_name: env!("CARGO_PKG_NAME"),
		author: env!("CARGO_PKG_AUTHORS"),
	};

	let config = get_config(&opt, &version_info)?;

	rpc::run(&opt, &config)?;

	Ok(())
}

fn init_logger(log: &Option<String>) -> errors::Result<()> {
	let mut builder = env_logger::Builder::new();

	builder.filter(None, log::LevelFilter::Info);

	if let Ok(rust_log) = std::env::var("RUST_LOG") {
		builder.parse_filters(&rust_log);
	}

	if let Some(log) = log {
		builder.parse_filters(&log);
	}

	builder.try_init().map_err(|_e| "Init logger error")?;

	Ok(())
}
