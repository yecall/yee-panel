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

use log::error;
use std::io;

const MAX_PAYLOAD: usize = 15 * 1024 * 1024;

type Metadata = crate::rpc::metadata::Metadata;
type RpcHandler = pubsub::PubSubHandler<Metadata>;

pub fn start_http(addr: &std::net::SocketAddr, io: RpcHandler) -> io::Result<http::Server> {
    http::ServerBuilder::new(io)
        .threads(4)
        .health_api(("/health", "system_health"))
        .rest_api(http::RestApi::Unsecure)
        .cors(http::DomainsValidation::Disabled)
        .max_request_body_size(MAX_PAYLOAD)
        .start_http(addr)
}

/// Start WS server listening on given address.
pub fn start_ws(addr: &std::net::SocketAddr, io: RpcHandler) -> io::Result<ws::Server> {
    ws::ServerBuilder::with_meta_extractor(io, |context: &ws::RequestContext| {
        Metadata::new(context.sender())
    })
    .max_payload(MAX_PAYLOAD)
    .start(addr)
    .map_err(|err| match err {
        ws::Error(ws::ErrorKind::Io(io), _) => io,
        ws::Error(ws::ErrorKind::ConnectionClosed, _) => io::ErrorKind::BrokenPipe.into(),
        ws::Error(e, _) => {
            error!("{}", e);
            io::ErrorKind::Other.into()
        }
    })
}
