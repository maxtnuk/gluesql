use crate::*;

test_case!(basic, async move {
    run!(
        r#"
CREATE TABLE Test (
    id INTEGER,
    num INTEGER,
    name TEXT
)"#
    );
    run!("INSERT INTO Test (id, num, name) VALUES (1, 2, \"Hello\")");
    run!("INSERT INTO Test (id, num, name) VALUES (1, 9, \"World\")");
    run!("INSERT INTO Test (id, num, name) VALUES (3, 4, \"Great\"), (4, 7, \"Job\")");

    use Value::*;

    test!(
        Ok(select!(
            id  | num | name
            I64 | I64 | Str;
            1     2     "Hello".to_owned();
            1     9     "World".to_owned();
            3     4     "Great".to_owned();
            4     7     "Job".to_owned()
        )),
        "SELECT id, num, name FROM Test"
    );

    count!(4, "SELECT * FROM Test");

    run!("UPDATE Test SET id = 2");

    let test_cases = vec![
        (Ok(select!(id; I64; 2; 2; 2; 2)), "SELECT id FROM Test"),
        (
            Ok(select!(id | num; I64 | I64; 2 2; 2 9; 2 4; 2 7)),
            "SELECT id, num FROM Test",
        ),
    ];

    for (expected, sql) in test_cases.into_iter() {
        test!(expected, sql);
    }
});
