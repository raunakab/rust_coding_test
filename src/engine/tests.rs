use crate::engine::run;

#[test]
fn basic() {
    run("assets/transactions.csv").unwrap();
}
