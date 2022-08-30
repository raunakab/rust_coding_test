use std::path::PathBuf;

use crate::engine::core::Core;
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

pub fn run<P>(src: P) -> EngineResult<()>
where
    P: Into<PathBuf>,
{
    let src = src.into();
    let transactions = deserializer::deserialize(src)?;
    let mut core = Core::default();
    core.process_batch(transactions);
    let clients = core.clients();
    serializer::serialize(clients);
    Ok(())
}
