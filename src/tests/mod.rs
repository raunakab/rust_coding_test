use crate::engine::Engine;
use crate::transactions::Charge;
use crate::transactions::Transaction;

#[test]
fn test() {
    let transactions = vec![Transaction::Deposit(Charge {
        client: 0,
        tx: 0,
        amount: 100,
    })];
    #[cfg_attr(not(test), allow(unused))]
    let engine = transactions.into_iter().fold(
        Engine::default(),
        |mut engine, transaction| {
            engine
                .process(transaction)
                .unwrap_or_else(|_| println!("Transaction rejected."));
            engine
        },
    );

    #[cfg(test)]
    println!("{:#?}", engine);
}
