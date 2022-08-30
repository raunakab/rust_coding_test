use std::env;
use std::process::ExitCode;

pub mod client;
pub mod engine;
pub mod transaction;

pub mod types {
    pub type EngineResult<T> = Result<T, &'static str>;
    pub type ClientId = u16;
    pub type Amount = f64;
    pub type TransactionId = u32;
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    let args = &*args;
    match args {
        [_, src] => engine::batch(src),
        _ => Err("Oops, this binary requires one argument (which is the relative path to the input file)."),
    }.map_or(ExitCode::FAILURE, |()| ExitCode::SUCCESS)
}
