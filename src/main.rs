pub mod client;
pub mod engine;
#[cfg(test)]
mod tests;
pub mod transactions;

pub mod types {
    pub type EngineResult<T> = Result<T, &'static str>;
    pub type ClientId = u16;
    pub type Amount = u64;
    pub type TransactionId = u32;
}

fn main() {}
