use connectorx::prelude::*;
use connectorx::sources::oracle::OracleSource;
use connectorx::sql::CXQuery;
use std::env;

#[test]
#[ignore]
fn test_types() {
    let _ = env_logger::builder().is_test(true).try_init();
    let dburl = env::var("ORACLE_URL").unwrap();
    let mut source = OracleSource::new(&dburl, 1).unwrap();
    #[derive(Debug, PartialEq)]
    struct Row(
        i64, // test_int NUMBER(7)
        Option<String>, // test_char CHAR(5)
        Option<f64>, // test_float FLOAT(53)
    );

    source.set_queries(&[CXQuery::naked("select * from test_table")]);
    source.fetch_metadata().unwrap();
    let mut partitions = source.partition().unwrap();
    assert!(partitions.len() == 1);
    let mut partition = partitions.remove(0);
    partition.result_rows().expect("run query");
    assert_eq!(5, partition.nrows());
    assert_eq!(3, partition.ncols());

    let mut parser = partition.parser().unwrap();

    let mut rows: Vec<Row> = Vec::new();
    loop {
        let (n, is_last) = parser.fetch_next().unwrap();
        for _i in 0..n {
            rows.push(Row(
                parser.produce().unwrap(),
                parser.produce().unwrap(),
                parser.produce().unwrap(),
            ));
        }
        if is_last {
            break;
        }
    }

    assert_eq!(
        vec![
            Row(
                1,
                Some("str1".to_string()),
                Some(1.1)
            ),
            Row(
                2,
                Some("str2".to_string()),
                Some(2.2)
            ),
            Row(
                2333,
                None,
                None,
            ),
            Row(
                4,
                None,
                Some(-4.44)
            ),
            Row(
                5,
                Some("str05".to_string()),
                None
            ),
        ],
        rows
    );
}
