#[test]
fn basic_transaction() {
    let id = 1;
    let clients = vec![(id, client!([id, 0.0, 0.0, false]))]
        .into_iter()
        .collect();
    let mut core = super::Core {
        clients,
        ..Default::default()
    };
    process!([transaction!(["deposit", 1, 1, 1.0])] -> core);
    assert_clients_eq!(core == [client!([1, 0.0, 0.0, false])]);
}

#[test]
fn create_new_client() {
    let mut core = super::Core::default();
    process!([transaction!(["deposit", 1, 1, 1.0])] -> core);
    assert_clients_eq!(core == [client!([1, 1.0, 0.0, false])]);
}

#[test]
fn dispute() {
    let mut core = super::Core::default();
    process!([
        transaction!(["deposit", 1, 1, 1.0]),
        transaction!(["dispute", 1, 1]),
        transaction!(["dispute", 1, 1]),
        transaction!(["dispute", 1, 1]),
        transaction!(["dispute", 1, 1]),
        transaction!(["dispute", 1, 1]),
        transaction!(["dispute", 1, 1]),
    ] -> core);
    assert_clients_eq!(core == [client!([1, 0.0, 1.0, false])]);
}

#[test]
#[should_panic]
fn dispute_non_existent_transaction() {
    let mut core = super::Core::default();
    process!([
        transaction!(["deposit", 1, 1, 1.0]),
        transaction!(["dispute", 1, 2]),
    ] -> core);
}

#[test]
fn resolve() {
    let mut core = super::Core::default();
    process!([
        transaction!(["deposit", 1, 1, 1.0]),
        transaction!(["dispute", 1, 1]),
        transaction!(["resolve", 1, 1]),
        transaction!(["resolve", 1, 1]),
        transaction!(["resolve", 1, 1]),
        transaction!(["resolve", 1, 1]),
        transaction!(["resolve", 1, 1]),
        transaction!(["resolve", 1, 1]),
        transaction!(["resolve", 1, 1]),
        transaction!(["resolve", 1, 1]),
        transaction!(["resolve", 1, 1]),
    ] -> core);
    assert_clients_eq!(core == [client!([1, 1.0, 0.0, false])]);
}

#[test]
#[should_panic]
fn resolve_non_existent_transaction() {
    let mut core = super::Core::default();
    process!([
        transaction!(["deposit", 1, 1, 1.0]),
        transaction!(["resolve", 1, 2]),
    ] -> core);
}

#[test]
fn resolve_undisputed_deposit() {
    let mut core = super::Core::default();
    process!([
        transaction!(["deposit", 1, 1, 1.0]),
        transaction!(["resolve", 1, 1]),
        transaction!(["resolve", 1, 1]),
        transaction!(["resolve", 1, 1]),
        transaction!(["resolve", 1, 1]),
        transaction!(["resolve", 1, 1]),
    ] -> core);
    assert_clients_eq!(core == [client!([1, 1.0, 0.0, false])]);
}

#[test]
#[should_panic]
fn dispute_non_deposit() {
    let mut core = super::Core::default();
    process!([
        transaction!(["deposit", 1, 1, 1.0]),
        transaction!(["withdrawal", 1, 2, 1.0]),
        transaction!(["dispute", 1, 2]),
    ] -> core);
}

#[test]
fn chargeback() {
    let mut core = super::Core::default();
    process!([
        transaction!(["deposit", 1, 1, 1.0]),
        transaction!(["dispute", 1, 1]),
        transaction!(["chargeback", 1, 1]),
    ] -> core);
    assert_clients_eq!(core == [client!([1, 0.0, 0.0, true])]);
}

#[test]
fn chargeback_non_disputed_transaction() {
    let mut core = super::Core::default();
    process!([
        transaction!(["deposit", 1, 1, 1.0]),
        transaction!(["chargeback", 1, 1]),
    ] -> core);
    assert_clients_eq!(core == [client!([1, 1.0, 0.0, false])]);
}

#[test]
#[should_panic]
fn chargeback_non_deposit() {
    let mut core = super::Core::default();
    process!([
        transaction!(["deposit", 1, 1, 1.0]),
        transaction!(["withdrawal", 1, 2, 1.0]),
        transaction!(["chargeback", 1, 2]),
    ] -> core);
    assert_clients_eq!(core == [client!([1, 1.0, 0.0, false])]);
}

#[test]
#[should_panic]
fn transaction_after_chargeback() {
    let mut core = super::Core::default();
    process!([
        transaction!(["deposit", 1, 1, 2.0]),
        transaction!(["deposit", 1, 2, 1.0]),
        transaction!(["dispute", 1, 2]),
        transaction!(["chargeback", 1, 2]),
        transaction!(["deposit", 1, 3, 1.0]),
    ] -> core);
    assert_clients_eq!(core == [client!([1, 2.0, 0.0, true])]);
}
