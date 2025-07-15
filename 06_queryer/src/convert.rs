use sqlparser::ast::{Expr as SqlExpr, Offset as SqlOffset, Statement};

pub struct Sql<'a> {
    pub(crate) selection: Vec<Expr>,
    pub(crate) condition: Option<Expr>,
    pub(crate) source: &'a str,
    pub(crate) order_by: Vec<(String, bool)>,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<usize>,
}

impl<'a> TryFrom<&'a Statement> for Sql<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a Statement) -> Result<Self, Self::Error> {
        match sql {
            // 目前我们只关心 query (select ... from ... where ...)
            Statement::Query(id)=>{
                let Select {
                    from: table_with_joins,
                    selection:where_clause,
                    projection,
                }
            }
        }
    }
}