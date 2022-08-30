use crate::engine::Core;

#[test]
fn basic() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 100.0]),
        transaction!(Withdrawal @ [0, 1, 10.0]),
    ];
    let mut engine = Core::default();
    engine.process_batch(transactions);
    let clients = engine.clients();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 90.0, 0.0, false });
}

#[test]
fn basic_2() {
    let transactions = vec![
        transaction!(Deposit @ [1, 1, 10.0]),
        transaction!(Deposit @ [2, 2, 20.0]),
        transaction!(Deposit @ [1, 3, 20.0]),
        transaction!(Withdrawal @ [1, 4, 15.0]),
        transaction!(Withdrawal @ [2, 5, 30.0]),
    ];
    let mut engine = Core::default();
    engine.process_batch(transactions);
    let clients = engine.clients();
    let client1 = clients.first().unwrap();
    let client2 = clients.last().unwrap();
    assert_eq!(clients.len(), 2);
    assert_client!(client1 == { 1, 15.0, 0.0, false });
    assert_client!(client2 == { 2, 20.0, 0.0, false });
}

#[test]
fn too_large_withdrawal() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 5.0]),
        transaction!(Withdrawal @ [0, 1, 10.0]),
    ];
    let mut engine = Core::default();
    engine.process_batch(transactions);
    let clients = engine.clients();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 5.0, 0.0, false });
}

#[test]
fn too_large_withdrawal_2() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 10.0]),
        transaction!(Withdrawal @ [0, 1, 4.0]),
        transaction!(Withdrawal @ [0, 2, 4.0]),
        transaction!(Withdrawal @ [0, 3, 4.0]),
    ];
    let mut engine = Core::default();
    engine.process_batch(transactions);
    let clients = engine.clients();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 2.0, 0.0, false });
}

#[test]
fn multiple_deposits_and_withdrawals() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 10.0]),
        transaction!(Withdrawal @ [0, 1, 4.0]),
        transaction!(Withdrawal @ [0, 2, 4.0]),
        transaction!(Deposit @ [0, 3, 2.0]),
        transaction!(Withdrawal @ [0, 4, 4.0]),
    ];
    let mut engine = Core::default();
    engine.process_batch(transactions);
    let clients = engine.clients();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 0.0, 0.0, false });
}

#[test]
fn basic_dispute() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 10.0]),
        transaction!(Dispute @ [0, 0]),
    ];
    let mut engine = Core::default();
    engine.process_batch(transactions);
    let clients = engine.clients();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 0.0, 10.0, false });
}

#[test]
fn dispute_non_existent_tx() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 10.0]),
        transaction!(Dispute @ [0, 1]),
    ];
    let mut engine = Core::default();
    engine.process_batch(transactions);
    let clients = engine.clients();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 10.0, 0.0, false });
}

#[test]
fn dispute_after_withdrawal() {
    let transactions = vec![
        transaction!(Deposit @ [0, 0, 10.0]),
        transaction!(Withdrawal @ [0, 1, 1.0]),
        transaction!(Dispute @ [0, 0]),
    ];
    let mut engine = Core::default();
    engine.process_batch(transactions);
    let clients = engine.clients();
    let client = clients.first().unwrap();
    assert_eq!(clients.len(), 1);
    assert_client!(client == { 0, 9.0, 0.0, false });
}
