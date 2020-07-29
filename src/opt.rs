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

use std::path::PathBuf;

use structopt::StructOpt;

pub const DEFAULT_RPC_PORT: u16 = 10055;
pub const DEFAULT_WS_PORT: u16 = 10066;

#[derive(Debug, StructOpt, Clone)]
pub struct Opt {
    /// Specify HTTP RPC server TCP port
    #[structopt(long = "rpc-port", value_name = "PORT")]
    pub rpc_port: Option<u16>,

    /// Specify WebSockets RPC server TCP port
    #[structopt(long = "ws-port", value_name = "PORT")]
    pub ws_port: Option<u16>,

    /// Listen to all RPC interfaces (default is local)
    #[structopt(long = "rpc-external")]
    pub rpc_external: bool,

    /// Listen to all Websocket interfaces (default is local)
    #[structopt(long = "ws-external")]
    pub ws_external: bool,

    /// Specify custom base path.
    #[structopt(
        long = "base-path",
        short = "b",
        value_name = "PATH",
        parse(from_os_str)
    )]
    pub base_path: Option<PathBuf>,

    /// Sets a custom logging filter
    #[structopt(short = "l", long = "log", value_name = "LOG_PATTERN")]
    pub log: Option<String>,
}
