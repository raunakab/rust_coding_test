use std::env;
use std::process::ExitCode;

#[cfg(test)]
macro_rules! assert_clients_eq {
    ($core:ident == [$($client:expr)*]) => {{
        let crate::engine::core::Core { clients, .. } = $core;
        let actual_clients = clients.into_values().collect::<Vec<_>>();
        #[allow(unused_mut)]
        let mut expected_clients = Vec::<crate::client::Client>::new();
        $(expected_clients.push($client);)*
        assert_eq!(actual_clients, expected_clients);
    }};
}

#[cfg(test)]
macro_rules! client {
    ([$client:expr, $available:expr, $held:expr, $locked:expr]) => {
        crate::client::Client::new($client, $available, $held, $locked)
    };
}

#[cfg(test)]
macro_rules! transaction {
    (["deposit", $client:expr, $tx:expr, $amount:expr]) => {
        crate::transaction::Transaction::Deposit(crate::transaction::Charge {
            client: $client,
            tx: $tx,
            amount: $amount,
        })
    };
    (["withdrawal", $client:expr, $tx:expr, $amount:expr]) => {
        crate::transaction::Transaction::Withdrawal(crate::transaction::Charge {
            client: $client,
            tx: $tx,
            amount: $amount,
        })
    };
    (["dispute", $client:expr, $tx:expr]) => {
        crate::transaction::Transaction::Dispute(crate::transaction::ChargeRef {
            client: $client,
            tx: $tx,
        })
    };
    (["resolve", $client:expr, $tx:expr]) => {
        crate::transaction::Transaction::Resolve(crate::transaction::ChargeRef {
            client: $client,
            tx: $tx,
        })
    };
    (["chargeback", $client:expr, $tx:expr]) => {
        crate::transaction::Transaction::Chargeback(crate::transaction::ChargeRef {
            client: $client,
            tx: $tx,
        })
    };
}

#[cfg(test)]
macro_rules! process {
    ([$($transaction:expr),*$(,)?] -> $core:expr) => {
        $($core.process($transaction).unwrap();)*
    };
}

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
        [_, src] => engine::run(src),
        _ => Err("Oops, this binary requires one argument (which is the relative path to the input file)."),
    }.map_or(ExitCode::FAILURE, |()| ExitCode::SUCCESS)
}
