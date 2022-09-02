use std::io;

use serde::Serialize;

use crate::client::Client;
use crate::types::Amount;
use crate::types::ClientId;

#[derive(Serialize)]
struct RawClient {
    client: ClientId,
    available: Amount,
    held: Amount,
    total: Amount,
    locked: bool,
}

impl From<&Client> for RawClient {
    fn from(client: &Client) -> Self {
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

pub fn serialize(clients: Vec<&Client>) {
    let stdout = io::stdout();
    let mut writer = csv::Writer::from_writer(stdout);
    clients.into_iter().for_each(|client| {
        let raw_client = client.into();
        writer.serialize::<RawClient>(raw_client).ok();
    });
}
