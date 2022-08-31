#[cfg(test)]
mod tests;

use std::io;
use std::io::Write;

use serde::Serialize;

use crate::client::Client;
use crate::types::Amount;
use crate::types::ClientId;

#[derive(Serialize)]
pub struct RawClient {
    client: ClientId,
    available: Amount,
    held: Amount,
    total: Amount,
    locked: bool,
}

impl From<Client> for RawClient {
    fn from(client: Client) -> Self {
        let available = client.available();
        let held = client.held();
        let total = client.total();
        let locked = client.locked();
        let client = client.id();
        RawClient {
            client,
            available,
            held,
            total,
            locked,
        }
    }
}

pub fn serialize(clients: Vec<Client>) {
    let mut writer = csv::Writer::from_writer(io::stdout());
    clients.into_iter().for_each(|client| {
        let raw_client = client.into();
        writer.serialize::<RawClient>(raw_client).ok();
    });
}

pub fn serialize_stream(clients: Vec<Client>) {
    let mut stdout = std::io::stdout().lock();
    clients
        .into_iter()
        .for_each(|client| {
            let id = client.id();
            let available = client.available();
            let held = client.held();
            let total = client.total();
            let locked = client.locked();
            stdout.write_fmt(format_args!("id: {}, av: {}, held: {}, tot: {}, locked: {}\r\n", id, available, held, total, locked)).ok();
        });
}
