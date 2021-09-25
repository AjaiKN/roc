use crate::{
    lang::core::{
        expr::{expr2::Expr2, expr2_to_string::expr2_to_string},
        pattern::Pattern2,
    },
    pool::pool::{NodeId, Pool},
};

// A top level definition, not inside a function. For example: `main = "Hello, world!"`
#[derive(Debug)]
pub enum Def2 {
    // ValueDef example: `main = "Hello, world!"`. identifier -> `main`, expr -> "Hello, world!"
    ValueDef {
        identifier_id: NodeId<Pattern2>,
        expr_id: NodeId<Expr2>,
    },
    Blank,
}

pub type DefId = NodeId<Def2>;

pub fn def2_to_string(node_id: DefId, pool: &Pool) -> String {
    let mut full_string = String::new();
    let def2 = pool.get(node_id);

    match def2 {
        Def2::ValueDef {
            identifier_id,
            expr_id,
        } => {
            full_string.push_str(&format!(
                "Def2::ValueDef(identifier_id: >>{:?}), expr_id: >>{:?})",
                pool.get(*identifier_id),
                expr2_to_string(*expr_id, pool)
            ));
        }
        Def2::Blank => {
            full_string.push_str("Def2::Blank");
        }
    }

    full_string
}
