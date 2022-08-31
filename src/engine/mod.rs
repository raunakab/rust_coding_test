use std::io::Read;
use std::io::Write;
use std::net::SocketAddrV4;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::client::Client;
use crate::engine::core::Core;
use crate::engine::deserializer::deserialize;
use crate::engine::deserializer::deserialize_string;
use crate::engine::serializer::serialize;
use crate::engine::serializer::serialize_stream;
use crate::types::EngineResult;

#[cfg(test)]
macro_rules! transaction {
    ($action:ident @ [$id:literal, $tx:literal]) => {
        crate::transaction::Transaction::$action(
            crate::transaction::ChargeRef {
                client: $id,
                tx: $tx,
            },
        )
    };
    ($action:ident @ [$id:literal, $tx:literal, $amount:literal]) => {
        crate::transaction::Transaction::$action(crate::transaction::Charge {
            client: $id,
            tx: $tx,
            amount: $amount,
        })
    };
}

#[cfg(test)]
macro_rules! assert_client {
    ($client:ident == { $id:literal, $available:literal, $held:literal, $locked:literal }) => {{
        assert_eq!($client.id(), $id);
        assert_eq!($client.available(), $available);
        assert_eq!($client.held(), $held);
        assert_eq!($client.locked(), $locked);
    }};
}

mod core;
mod deserializer;
mod serializer;
#[cfg(test)]
mod tests;

pub fn batch<P>(src: P) -> EngineResult<()>
where
    P: Into<PathBuf>,
{
    let src = src.into();
    let transactions = deserialize(src)?;
    let mut core = Core::default();
    core.process_batch(transactions);
    let clients = core.clients();
    serialize(clients);
    Ok(())
}

pub fn stream() -> ! {
    let addr = "127.0.0.1:8080".parse::<SocketAddrV4>().unwrap();
    let listener = TcpListener::bind(addr)
        .map_err(|_| "Unable to bind to the address.")
        .unwrap();
    let core = Arc::new(Mutex::new(Core::default()));
    listener.incoming().into_iter().for_each(|tcp_stream| {
        tcp_stream
            .map(|mut tcp_stream| {
                let core = core.clone();
                thread::spawn(move || {
                    handle(&mut tcp_stream, core).map_or_else(
                        |err| {
                            let buf = err.to_string();
                            let buf = buf.as_bytes();
                            tcp_stream.write(buf).ok();
                        },
                        |()| (),
                    );
                })
            })
            .ok();
    });
    unreachable!("Never ending server...")
}

fn handle(
    tcp_stream: &mut TcpStream,
    core: Arc<Mutex<Core>>,
) -> EngineResult<()> {
    let mut buf = String::new();
    tcp_stream
        .read_to_string(&mut buf)
        .map_err(|_| "Unable to read tcp-stream to string.")?;
    let transaction = deserialize_string(buf)?;
    let mut core = core
        .lock()
        .map_err(|_| "Unable to get access to the locked core engine.")?;
    core.process(transaction)?;
    let clients: Vec<Client> =
        core.clients_ref().into_iter().cloned().collect();
    drop(core);
    serialize_stream(clients);
    Ok(())
}
