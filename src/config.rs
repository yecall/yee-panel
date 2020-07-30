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

use app_dirs::{AppDataType, AppInfo};
use log::trace;
use serde_derive::{Deserialize, Serialize};

use crate::errors;
use crate::opt::Opt;

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

	Ok(conf)
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
