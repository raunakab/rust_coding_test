use crate::transaction::Charge;
use crate::transaction::ChargeRef;
use crate::transaction::Transaction;

const BASE: &'static str = "src/engine/deserializer/tests";

fn to_src(name: &'static str, valid: bool) -> String {
    let validity = match valid {
        true => "valid",
        false => "invalid",
    };

    format!("{}/{}.{}", BASE, validity, name)
}

#[test]
fn deserialize() {
    let src = to_src("deposit.csv", true);
    super::deserialize(src, |transaction| {
        assert_eq!(
            transaction,
            Transaction::Deposit(Charge {
                client: 1,
                tx: 1,
                amount: 1.0
            })
        )
    })
    .unwrap();

    let src = to_src("withdrawal.csv", true);
    super::deserialize(src, |transaction| {
        assert_eq!(
            transaction,
            Transaction::Withdrawal(Charge {
                client: 1,
                tx: 1,
                amount: 1.0
            })
        )
    })
    .unwrap();

    let src = to_src("dispute.csv", true);
    super::deserialize(src, |transaction| {
        assert_eq!(
            transaction,
            Transaction::Dispute(ChargeRef { client: 1, tx: 1 })
        )
    })
    .unwrap();

    let src = to_src("resolve.csv", true);
    super::deserialize(src, |transaction| {
        assert_eq!(
            transaction,
            Transaction::Resolve(ChargeRef { client: 1, tx: 1 })
        )
    })
    .unwrap();

    let src = to_src("chargeback.csv", true);
    super::deserialize(src, |transaction| {
        assert_eq!(
            transaction,
            Transaction::Chargeback(ChargeRef { client: 1, tx: 1 })
        )
    })
    .unwrap();
}

#[test]
fn deserialize_invalid() {
    let src = to_src("deposit.csv", false);
    super::deserialize(src, |_| unreachable!("No iterations should be performed... Panic if an iteration occurs.")).unwrap();
}
