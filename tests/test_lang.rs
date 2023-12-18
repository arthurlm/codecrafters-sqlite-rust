use sqlite_starter_rust::lang::*;

#[test]
fn test_debug() {
    assert_eq!(
        format!("{:?}", select(&["col1", "col2"], "my_table")),
        "Select { columns: [\"col1\", \"col2\"], table_name: \"my_table\", where: None }"
    );
    assert_eq!(
        format!(
            "{:?}",
            select_where(&["col1", "col2"], "my_table", "foo", "bar")
        ),
        "Select { \
            columns: [\"col1\", \"col2\"], \
            table_name: \"my_table\", \
            where: Some(WhereClause { column_name: \"foo\", value: \"bar\" }) \
         }"
    );
    assert_eq!(
        format!("{:?}", ColumnDefinition::new("id", "integer")),
        "ColumnDefinition { name: \"id\", column_type: \"integer\", \
            is_primary_key: false, is_auto_increment: false \
         }"
    );
}

#[test]
fn test_identifier_valid() {
    identifier("hello").unwrap();
    identifier("heLLo").unwrap();
    identifier("h4").unwrap();
    identifier("_").unwrap();
    identifier("column_WITH_42long_NaMe").unwrap();
    identifier("'hello world'").unwrap();
    identifier("\"hello world\"").unwrap();
}

#[test]
fn test_identifier_invalid() {
    identifier("").unwrap_err();
    identifier(" ").unwrap_err();
    identifier("he llo").unwrap_err();
    identifier("4").unwrap_err();
    identifier("4lo").unwrap_err();
    identifier("'hello world").unwrap_err();
    identifier("\"hello world").unwrap_err();
}

#[test]
fn test_raw_string_valid() {
    raw_string("''").unwrap();
    raw_string("\"\"").unwrap();
    raw_string("'hello world that cont4ain5 w3eird chars !'").unwrap();
    raw_string("\"hello world that cont4ain5 w3eird chars !\"").unwrap();
}

#[test]
fn test_raw_string_invalid() {
    raw_string("").unwrap_err();

    // Single quote
    raw_string("'").unwrap_err();
    raw_string("'''").unwrap_err();
    raw_string("' '  '").unwrap_err();

    // Double quote
    raw_string("\"").unwrap_err();
    raw_string("\"\"\"").unwrap_err();
    raw_string("\" \"  \"").unwrap_err();

    // No quote
    raw_string("hello").unwrap_err();
}

#[test]
fn test_valid_select() {
    // Test single column sequence
    assert_eq!(
        parse_sql("select toto from foo").unwrap(),
        select(&["toto"], "foo")
    );
    assert_eq!(
        parse_sql("SELECT column_WITH_42long_NaMe FROM Bar").unwrap(),
        select(&["column_WITH_42long_NaMe"], "Bar")
    );

    // Test multi column sequence
    assert_eq!(
        parse_sql("select c1,c2 from foo").unwrap(),
        select(&["c1", "c2"], "foo")
    );
    assert_eq!(
        parse_sql("SELECT 'C1'  ,  \"C2\" ,   C3 , C4 FROM 'FOO'").unwrap(),
        select(&["C1", "C2", "C3", "C4"], "FOO")
    );

    // Test with where statement
    assert_eq!(
        parse_sql("select c1,c2 from foo where x='bar'").unwrap(),
        select_where(&["c1", "c2"], "foo", "x", "bar")
    );
    assert_eq!(
        parse_sql(
            "sELECt   'c1',   c2  \n\
             fROm  \n\t  \"foo\" \n\
             wHERe   \n
                'x'  \n = \n\t  'bar' \n
             "
        )
        .unwrap(),
        select_where(&["c1", "c2"], "foo", "x", "bar")
    );
}

#[test]
fn test_valid_create_table() {
    // Single column table
    assert_eq!(
        parse_sql("create table my_table(key string)").unwrap(),
        create_table("my_table", &[ColumnDefinition::new("key", "string")])
    );
    assert_eq!(
        parse_sql("CREATE    TABLE    my_table   \n  \n   (  \n \n   key string \n )").unwrap(),
        create_table("my_table", &[ColumnDefinition::new("key", "string")])
    );

    // Multi column table with column flag
    assert_eq!(
        parse_sql(
            "create table my_table(id integer primary key autoincrement,name text,color text)"
        )
        .unwrap(),
        create_table(
            "my_table",
            &[
                ColumnDefinition::new("id", "integer")
                    .primary_key()
                    .auto_increment(),
                ColumnDefinition::new("name", "text"),
                ColumnDefinition::new("color", "text")
            ]
        )
    );
    assert_eq!(
        parse_sql(
            "create   table    my_table   \n\
             (   \n\
                id    integer    PRIMARY    KEY  AUTOINCREMENT  ,  \n\
                name   text , \n\
                color text  \n\
             )"
        )
        .unwrap(),
        create_table(
            "my_table",
            &[
                ColumnDefinition::new("id", "integer")
                    .primary_key()
                    .auto_increment(),
                ColumnDefinition::new("name", "text"),
                ColumnDefinition::new("color", "text")
            ]
        )
    );
}

#[test]
fn test_valid_create_index() {
    // Single column index
    assert_eq!(
        parse_sql("create index my_index on my_table(col1)").unwrap(),
        create_index("my_index", "my_table", &["col1"])
    );
    assert_eq!(
        parse_sql(
            "CREATE   INDEX   'my_index'
             On   \"my_table\"  (
                'col1'
            )"
        )
        .unwrap(),
        create_index("my_index", "my_table", &["col1"])
    );

    // Multi column index
    assert_eq!(
        parse_sql("create index my_index on my_table(col1,col2,col3)").unwrap(),
        create_index("my_index", "my_table", &["col1", "col2", "col3"])
    );
    assert_eq!(
        parse_sql(
            "CREATE   INDEX   'my_index'
             On   \"my_table\"  (
                'col1' ,
                col2   ,
                \"col3\"
            )"
        )
        .unwrap(),
        create_index("my_index", "my_table", &["col1", "col2", "col3"])
    );
}

fn check_err(input: &str) {
    parse_sql(input).unwrap_err();
}

#[test]
fn test_invalid_sql() {
    check_err("bad keyword");
}

#[test]
fn test_invalid_select() {
    // Missing keyword
    check_err("select");

    // Missing space
    check_err("selectfrom");
    check_err("selecttotofromfoo");

    // Missing values
    check_err("select   from   ");
    check_err("select   from   foo");
    check_err("select   from   foo");

    // Missing separator
    check_err("select c1 c2  from   foo");
}
