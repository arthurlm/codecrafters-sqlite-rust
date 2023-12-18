use std::collections::HashMap;

use sqlite_starter_rust::{error::SqliteError, lang::*, schema_def::SchemaDefinition};

const TABLE_SQL_DEF: &str =
    "create table bar (name text, id integer primary key, description text)";

#[test]
fn test_derive() {
    // Debug
    assert_eq!(
        format!(
            "{:?}",
            SchemaDefinition {
                table_name: "foo",
                pk_index: 0,
                column_indexes: HashMap::from([("id", 0)])
            }
        ),
        "SchemaDefinition { table_name: \"foo\", pk_index: 0, column_indexes: {\"id\": 0} }"
    );
}

#[test]
fn test_from_bad_sql() {
    let sql = parse_sql("select foo from bar").unwrap();
    assert_eq!(
        SchemaDefinition::try_from(&sql),
        Err(SqliteError::InvalidSqliteSchema)
    );
}

#[test]
fn test_missing_pk() {
    let sql = parse_sql("create table foo(id text)").unwrap();
    assert_eq!(
        SchemaDefinition::try_from(&sql),
        Err(SqliteError::MissingPrimaryKey)
    );
}

#[test]
fn test_convert_ok() {
    let sql = parse_sql("create table foo(id text primary key)").unwrap();
    let schema = SchemaDefinition::try_from(&sql).unwrap();
    assert_eq!(
        schema,
        SchemaDefinition {
            table_name: "foo",
            pk_index: 0,
            column_indexes: HashMap::from([("id", 0)])
        }
    );

    let sql = parse_sql(TABLE_SQL_DEF).unwrap();
    let schema = SchemaDefinition::try_from(&sql).unwrap();
    assert_eq!(
        schema,
        SchemaDefinition {
            table_name: "bar",
            pk_index: 1,
            column_indexes: HashMap::from([("name", 0), ("id", 1), ("description", 2)])
        }
    );
}

#[test]
fn test_is_pk() {
    let sql = parse_sql(TABLE_SQL_DEF).unwrap();
    let schema = SchemaDefinition::try_from(&sql).unwrap();

    assert!(!schema.is_pk("invalid"));
    assert!(!schema.is_pk("name"));
    assert!(schema.is_pk("id"));
    assert!(!schema.is_pk("description"));
}

#[test]
fn test_column_index() {
    let sql = parse_sql(TABLE_SQL_DEF).unwrap();
    let schema = SchemaDefinition::try_from(&sql).unwrap();

    assert_eq!(schema.column_index("invalid"), None);
    assert_eq!(schema.column_index("name"), Some(0));
    assert_eq!(schema.column_index("id"), Some(1));
    assert_eq!(schema.column_index("description"), Some(2));
}
