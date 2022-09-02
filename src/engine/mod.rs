use std::path::PathBuf;

use crate::engine::core::Core;
use crate::types::EngineResult;

mod core;
mod deserializer;
mod serializer;

pub fn run<P>(src: P) -> EngineResult<()>
where
    P: Into<PathBuf>,
{
    let mut core = Core::default();
    deserializer::deserialize(src, |transaction| {
        core.process(transaction).ok();
    })
    .ok();
    let clients = core.clients();
    serializer::serialize(clients);
    Ok(())
}
