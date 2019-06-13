// Copyright 2019 Mats Kindahl
//
// Licensed under the Apache License, Version 2.0 (the "License"); you
// may not use this file except in compliance with the License.  You
// may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied.  See the License for the specific language governing
// permissions and limitations under the License.

extern crate bytes;
extern crate futures;
extern crate tokio;

use std::net::SocketAddr;
use tokio::codec::BytesCodec;
use tokio::net::{UdpFramed, UdpSocket};
use tokio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listen_address = "0.0.0.0:4711".parse::<SocketAddr>()?;
    let socket = UdpSocket::bind(&listen_address)?;
    let peers: Vec<SocketAddr> = vec!["192.168.1.136:8080".parse()?, "192.168.1.136:8081".parse()?];
    let (mut writer, reader) = UdpFramed::new(socket, BytesCodec::new()).split();
    let forwarder = reader.for_each(move |(bytes, _from)| {
        for peer in peers.iter() {
            writer.start_send((bytes.clone().into(), peer.clone()))?;
        }
        writer.poll_complete()?;
        Ok(())
    });

    tokio::run({
        forwarder
            .map_err(|err| println!("Error: {}", err))
            .map(|_| ())
    });
    Ok(())
}
