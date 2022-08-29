use crate::engine::Engine;
use crate::transactions::Charge;
use crate::transactions::Record;
use crate::transactions::Transaction;

macro_rules! transaction {
    ($action:ident @ [$id:literal, $tx:literal]) => (Transaction::$action(Record {
        client: $id,
        tx: $tx,
    }));
    ($action:ident @ [$id:literal, $tx:literal, $amount:literal]) => (Transaction::$action(Charge {
        client: $id,
        tx: $tx,
        amount: $amount,
    }));
}

macro_rules! assert_client {
    ($client:ident == { $id:literal, $available:literal, $held:literal, $locked:literal }) => {{
        assert_eq!($client.id(), $id);
        assert_eq!($client.available(), $available);
        assert_eq!($client.held(), $held);
        assert_eq!($client.locked(), $locked);
    }};
}

#[test]
fn basic() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 100]),
        transaction!(Withdrawal @ [0, 1, 10]),
    ];
    let mut engine = Engine::default();
    engine.process_batch(transactions);
    let clients = engine.clients().into_values().collect::<Vec<_>>();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 90, 0, false });
}

#[test]
fn basic_2() {
    let transactions = vec![
        transaction!(Deposit @ [1, 1, 10]),
        transaction!(Deposit @ [2, 2, 20]),
        transaction!(Deposit @ [1, 3, 20]),
        transaction!(Withdrawal @ [1, 4, 15]),
        transaction!(Withdrawal @ [2, 5, 30]),
    ];
    let mut engine = Engine::default();
    engine.process_batch(transactions);
    let clients = engine.clients().into_values().collect::<Vec<_>>();
    let client1 = clients.first().unwrap();
    let client2 = clients.last().unwrap();
    assert_eq!(clients.len(), 2);
    assert_client!(client1 == { 1, 15, 0, false });
    assert_client!(client2 == { 2, 20, 0, false });
}

#[test]
fn too_large_withdrawal() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 5]),
        transaction!(Withdrawal @ [0, 1, 10]),
    ];
    let mut engine = Engine::default();
    engine.process_batch(transactions);
    let clients = engine.clients().into_values().collect::<Vec<_>>();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 5, 0, false });
}

#[test]
fn too_large_withdrawal_2() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 10]),
        transaction!(Withdrawal @ [0, 1, 4]),
        transaction!(Withdrawal @ [0, 2, 4]),
        transaction!(Withdrawal @ [0, 3, 4]),
    ];
    let mut engine = Engine::default();
    engine.process_batch(transactions);
    let clients = engine.clients().into_values().collect::<Vec<_>>();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 2, 0, false });
}

#[test]
fn multiple_deposits_and_withdrawals() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 10]),
        transaction!(Withdrawal @ [0, 1, 4]),
        transaction!(Withdrawal @ [0, 2, 4]),
        transaction!(Deposit @ [0, 3, 2]),
        transaction!(Withdrawal @ [0, 4, 4]),
    ];
    let mut engine = Engine::default();
    engine.process_batch(transactions);
    let clients = engine.clients().into_values().collect::<Vec<_>>();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 0, 0, false });
}

#[test]
fn basic_dispute() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 10]),
        transaction!(Dispute @ [0, 0]),
    ];
    let mut engine = Engine::default();
    engine.process_batch(transactions);
    let clients = engine.clients().into_values().collect::<Vec<_>>();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 0, 10, false });
}

#[test]
fn dispute_non_existent_tx() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 10]),
        transaction!(Dispute @ [0, 1]),
    ];
    let mut engine = Engine::default();
    engine.process_batch(transactions);
    let clients = engine.clients().into_values().collect::<Vec<_>>();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 10, 0, false });
}

#[test]
fn dispute_after_withdrawal() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 10]),
        transaction!(Withdrawal @ [0, 1, 1]),
        transaction!(Dispute @ [0, 0]),
    ];
    let mut engine = Engine::default();
    engine.process_batch(transactions);
    let clients = engine.clients().into_values().collect::<Vec<_>>();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 9, 0, false });
}
