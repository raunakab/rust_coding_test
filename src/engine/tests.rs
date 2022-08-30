use crate::engine::batch;

#[test]
fn basic() {
    batch("assets/transactions.csv").unwrap();
}
