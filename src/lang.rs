peg::parser!(
    grammar sql_parser() for str {
        pub rule expression() -> SqlTree
            = select_statement() / create_table_statement()

        rule _ = quiet!{[' ' | '\t' | '\n']+}

        rule select_statement() -> SqlTree
            = "select" _ columns:(identifier() ++ (_* "," _*))
            _ "from" _ table_name:identifier()
            r#where:where_statement()?
            _* {
                SqlTree::Select { columns, table_name, r#where }
            }

        rule where_statement() -> WhereClause
            = _ "where" _ column_name:identifier() _* "=" _* value:raw_string() {
                WhereClause { column_name, value }
            }

        rule create_table_statement() -> SqlTree
            = "create" _ "table" _ table_name:identifier()
                _* "("
                _* columns_def:(column_definition() ++ (_* "," _*))
                _* ")" {
                SqlTree::CreateTable { table_name, columns_def }
            }

        rule column_definition() -> ColumnDefinition
            = name:identifier() _ column_type:identifier()
                pk:(_ "primary" _ "key")?
                ai:(_ "autoincrement")? {
                ColumnDefinition {
                    name,
                    column_type,
                    is_primary_key: pk.is_some(),
                    is_auto_increment: ai.is_some(),
                }
            }

        pub rule raw_string() -> String
            = "'" value:$([^'\'']*) "'" { value.to_string()}

        pub rule identifier() -> String
            = v:$(['a'..='z' | 'A'..='Z' | '_']+['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { v.to_string() }
        }
);

pub use sql_parser::{expression as parse_sql, identifier, raw_string};

#[derive(Debug, PartialEq, Eq)]
pub enum SqlTree {
    Select {
        columns: Vec<String>,
        table_name: String,
        r#where: Option<WhereClause>,
    },
    CreateTable {
        table_name: String,
        columns_def: Vec<ColumnDefinition>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct WhereClause {
    column_name: String,
    value: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub column_type: String,
    pub is_primary_key: bool,
    pub is_auto_increment: bool,
}

impl ColumnDefinition {
    pub fn new(name: &str, column_type: &str) -> Self {
        Self {
            name: name.to_string(),
            column_type: column_type.to_string(),
            is_primary_key: false,
            is_auto_increment: false,
        }
    }

    pub fn primary_key(mut self) -> Self {
        self.is_primary_key = true;
        self
    }

    pub fn auto_increment(mut self) -> Self {
        self.is_auto_increment = true;
        self
    }
}

pub fn select(columns: &[&str], table_name: &str) -> SqlTree {
    SqlTree::Select {
        columns: columns.iter().map(|c| c.to_string()).collect(),
        table_name: table_name.to_string(),
        r#where: None,
    }
}

pub fn select_where(
    columns: &[&str],
    table_name: &str,
    filter_column: &str,
    filter_value: &str,
) -> SqlTree {
    SqlTree::Select {
        columns: columns.iter().map(|c| c.to_string()).collect(),
        table_name: table_name.to_string(),
        r#where: Some(WhereClause {
            column_name: filter_column.to_string(),
            value: filter_value.to_string(),
        }),
    }
}

pub fn create_table(table_name: &str, columns: &[ColumnDefinition]) -> SqlTree {
    SqlTree::CreateTable {
        table_name: table_name.to_string(),
        columns_def: columns.to_vec(),
    }
}
