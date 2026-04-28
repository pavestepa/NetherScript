use crate::stmt::{SirStmt, stmt_count};

#[test]
fn counts_statements() {
    let stmts = vec![SirStmt::Break, SirStmt::Error];
    assert_eq!(stmt_count(&stmts), 2);
}
