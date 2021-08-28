use super::join;
use super::Opts;
use indoc::indoc;
use std::io;
use std::io::Seek;
use std::io::Write;
use std::str;

static NDJSON1: &str = indoc! {r#"
    {"id": 1, "sub_id": 11, "file":"ndjson1", "val":"both"}
    {"id": 2, "sub_id": 21, "file":"ndjson1", "val":"both/multi 1"}
    {"id": 2, "sub_id": 22, "file":"ndjson1", "val":"both/multi 1"}
    {"id": 3, "sub_id": 31, "file":"ndjson1", "val":"both/multi 2"}
    {"id": 4, "sub_id": 41, "file":"ndjson1", "val":"only 1"}
    {"id": 6, "sub_id": 62, "file":"ndjson1", "val":"both/multi"}
    {"id": 6, "sub_id": 61, "file":"ndjson1", "val":"both/multi"}
    {"id":11, "sub_id":111, "file":"ndjson1", "val":"both"}
    {"id":12, "sub_id":121, "file":"ndjson1", "val":"both/multi 1"}
    {"id":12, "sub_id":122, "file":"ndjson1", "val":"both/multi 1"}
    {"id":13, "sub_id":131, "file":"ndjson1", "val":"both/multi 2"}
    {"id":14, "sub_id":141, "file":"ndjson1", "val":"only 1"}
    {"id":16, "sub_id":162, "file":"ndjson1", "val":"both/multi"}
    {"id":16, "sub_id":161, "file":"ndjson1", "val":"both/multi"}
"#};

static NDJSON2: &str = indoc! {r#"
    {"id": 1, "sub_id": 11, "file":"ndjson2", "val":"both"}
    {"id": 2, "sub_id": 21, "file":"ndjson2", "val":"both/multi 1"}
    {"id": 3, "sub_id": 31, "file":"ndjson2", "val":"both/multi 2"}
    {"id": 3, "sub_id": 32, "file":"ndjson2", "val":"both/multi 2"}
    {"id": 5, "sub_id": 51, "file":"ndjson2", "val":"only 2"}
    {"id": 6, "sub_id": 62, "file":"ndjson2", "val":"both/multi"}
    {"id": 6, "sub_id": 61, "file":"ndjson2", "val":"both/multi"}
    {"id":11, "sub_id":111, "file":"ndjson2", "val":"both"}
    {"id":12, "sub_id":121, "file":"ndjson2", "val":"both/multi 1"}
    {"id":13, "sub_id":131, "file":"ndjson2", "val":"both/multi 2"}
    {"id":13, "sub_id":132, "file":"ndjson2", "val":"both/multi 2"}
    {"id":15, "sub_id":151, "file":"ndjson2", "val":"only 2"}
    {"id":16, "sub_id":162, "file":"ndjson2", "val":"both/multi"}
    {"id":16, "sub_id":161, "file":"ndjson2", "val":"both/multi"}
"#};

#[test]
fn test_join() {
    let mut f1 = tempfile::tempfile().unwrap();
    write!(f1, "{}", NDJSON1).unwrap();
    f1.seek(io::SeekFrom::Start(0)).unwrap();

    let mut f2 = tempfile::tempfile().unwrap();
    write!(f2, "{}", NDJSON2).unwrap();
    f2.seek(io::SeekFrom::Start(0)).unwrap();

    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    join(
        f1,
        "id",
        f2,
        "id",
        fout,
        Opts {
            allow_no_key: false,
            merge: None,
        },
    )
    .unwrap();

    assert_eq!(
        indoc! {r#"
            [{"file":"ndjson1","id":1,"sub_id":11,"val":"both"},{"file":"ndjson2","id":1,"sub_id":11,"val":"both"}]
            [{"file":"ndjson1","id":2,"sub_id":21,"val":"both/multi 1"},{"file":"ndjson2","id":2,"sub_id":21,"val":"both/multi 1"}]
            [{"file":"ndjson1","id":2,"sub_id":22,"val":"both/multi 1"},{"file":"ndjson2","id":2,"sub_id":21,"val":"both/multi 1"}]
            [{"file":"ndjson1","id":3,"sub_id":31,"val":"both/multi 2"},{"file":"ndjson2","id":3,"sub_id":31,"val":"both/multi 2"}]
            [{"file":"ndjson1","id":3,"sub_id":31,"val":"both/multi 2"},{"file":"ndjson2","id":3,"sub_id":32,"val":"both/multi 2"}]
            [{"file":"ndjson1","id":6,"sub_id":62,"val":"both/multi"},{"file":"ndjson2","id":6,"sub_id":62,"val":"both/multi"}]
            [{"file":"ndjson1","id":6,"sub_id":62,"val":"both/multi"},{"file":"ndjson2","id":6,"sub_id":61,"val":"both/multi"}]
            [{"file":"ndjson1","id":6,"sub_id":61,"val":"both/multi"},{"file":"ndjson2","id":6,"sub_id":62,"val":"both/multi"}]
            [{"file":"ndjson1","id":6,"sub_id":61,"val":"both/multi"},{"file":"ndjson2","id":6,"sub_id":61,"val":"both/multi"}]
            [{"file":"ndjson1","id":11,"sub_id":111,"val":"both"},{"file":"ndjson2","id":11,"sub_id":111,"val":"both"}]
            [{"file":"ndjson1","id":12,"sub_id":121,"val":"both/multi 1"},{"file":"ndjson2","id":12,"sub_id":121,"val":"both/multi 1"}]
            [{"file":"ndjson1","id":12,"sub_id":122,"val":"both/multi 1"},{"file":"ndjson2","id":12,"sub_id":121,"val":"both/multi 1"}]
            [{"file":"ndjson1","id":13,"sub_id":131,"val":"both/multi 2"},{"file":"ndjson2","id":13,"sub_id":131,"val":"both/multi 2"}]
            [{"file":"ndjson1","id":13,"sub_id":131,"val":"both/multi 2"},{"file":"ndjson2","id":13,"sub_id":132,"val":"both/multi 2"}]
            [{"file":"ndjson1","id":16,"sub_id":162,"val":"both/multi"},{"file":"ndjson2","id":16,"sub_id":162,"val":"both/multi"}]
            [{"file":"ndjson1","id":16,"sub_id":162,"val":"both/multi"},{"file":"ndjson2","id":16,"sub_id":161,"val":"both/multi"}]
            [{"file":"ndjson1","id":16,"sub_id":161,"val":"both/multi"},{"file":"ndjson2","id":16,"sub_id":162,"val":"both/multi"}]
            [{"file":"ndjson1","id":16,"sub_id":161,"val":"both/multi"},{"file":"ndjson2","id":16,"sub_id":161,"val":"both/multi"}]
        "#},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_join_with_merging1() {
    let mut f1 = tempfile::tempfile().unwrap();
    write!(f1, "{}", NDJSON1).unwrap();
    f1.seek(io::SeekFrom::Start(0)).unwrap();

    let mut f2 = tempfile::tempfile().unwrap();
    write!(f2, "{}", NDJSON2).unwrap();
    f2.seek(io::SeekFrom::Start(0)).unwrap();

    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    join(
        f1,
        "id",
        f2,
        "id",
        fout,
        Opts {
            allow_no_key: false,
            merge: Some(1),
        },
    )
    .unwrap();

    assert_eq!(
        indoc! {r#"
            {"file":"ndjson1","id":1,"sub_id":11,"val":"both"}
            {"file":"ndjson1","id":2,"sub_id":21,"val":"both/multi 1"}
            {"file":"ndjson1","id":2,"sub_id":22,"val":"both/multi 1"}
            {"file":"ndjson1","id":3,"sub_id":31,"val":"both/multi 2"}
            {"file":"ndjson1","id":3,"sub_id":31,"val":"both/multi 2"}
            {"file":"ndjson1","id":6,"sub_id":62,"val":"both/multi"}
            {"file":"ndjson1","id":6,"sub_id":62,"val":"both/multi"}
            {"file":"ndjson1","id":6,"sub_id":61,"val":"both/multi"}
            {"file":"ndjson1","id":6,"sub_id":61,"val":"both/multi"}
            {"file":"ndjson1","id":11,"sub_id":111,"val":"both"}
            {"file":"ndjson1","id":12,"sub_id":121,"val":"both/multi 1"}
            {"file":"ndjson1","id":12,"sub_id":122,"val":"both/multi 1"}
            {"file":"ndjson1","id":13,"sub_id":131,"val":"both/multi 2"}
            {"file":"ndjson1","id":13,"sub_id":131,"val":"both/multi 2"}
            {"file":"ndjson1","id":16,"sub_id":162,"val":"both/multi"}
            {"file":"ndjson1","id":16,"sub_id":162,"val":"both/multi"}
            {"file":"ndjson1","id":16,"sub_id":161,"val":"both/multi"}
            {"file":"ndjson1","id":16,"sub_id":161,"val":"both/multi"}
        "#},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_join_with_merging2() {
    let mut f1 = tempfile::tempfile().unwrap();
    write!(f1, "{}", NDJSON1).unwrap();
    f1.seek(io::SeekFrom::Start(0)).unwrap();

    let mut f2 = tempfile::tempfile().unwrap();
    write!(f2, "{}", NDJSON2).unwrap();
    f2.seek(io::SeekFrom::Start(0)).unwrap();

    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    join(
        f1,
        "id",
        f2,
        "id",
        fout,
        Opts {
            allow_no_key: false,
            merge: Some(2),
        },
    )
    .unwrap();

    assert_eq!(
        indoc! {r#"
            {"file":"ndjson2","id":1,"sub_id":11,"val":"both"}
            {"file":"ndjson2","id":2,"sub_id":21,"val":"both/multi 1"}
            {"file":"ndjson2","id":2,"sub_id":21,"val":"both/multi 1"}
            {"file":"ndjson2","id":3,"sub_id":31,"val":"both/multi 2"}
            {"file":"ndjson2","id":3,"sub_id":32,"val":"both/multi 2"}
            {"file":"ndjson2","id":6,"sub_id":62,"val":"both/multi"}
            {"file":"ndjson2","id":6,"sub_id":61,"val":"both/multi"}
            {"file":"ndjson2","id":6,"sub_id":62,"val":"both/multi"}
            {"file":"ndjson2","id":6,"sub_id":61,"val":"both/multi"}
            {"file":"ndjson2","id":11,"sub_id":111,"val":"both"}
            {"file":"ndjson2","id":12,"sub_id":121,"val":"both/multi 1"}
            {"file":"ndjson2","id":12,"sub_id":121,"val":"both/multi 1"}
            {"file":"ndjson2","id":13,"sub_id":131,"val":"both/multi 2"}
            {"file":"ndjson2","id":13,"sub_id":132,"val":"both/multi 2"}
            {"file":"ndjson2","id":16,"sub_id":162,"val":"both/multi"}
            {"file":"ndjson2","id":16,"sub_id":161,"val":"both/multi"}
            {"file":"ndjson2","id":16,"sub_id":162,"val":"both/multi"}
            {"file":"ndjson2","id":16,"sub_id":161,"val":"both/multi"}
        "#},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_join_different_key() {
    let mut f1 = tempfile::tempfile().unwrap();
    write!(f1, "{}", NDJSON1).unwrap();
    f1.seek(io::SeekFrom::Start(0)).unwrap();

    let mut f2 = tempfile::tempfile().unwrap();
    write!(f2, "{}", NDJSON2.to_string().replace(r#""id""#, r#""id2""#)).unwrap();
    f2.seek(io::SeekFrom::Start(0)).unwrap();

    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    join(
        f1,
        "id",
        f2,
        "id2",
        fout,
        Opts {
            allow_no_key: false,
            merge: None,
        },
    )
    .unwrap();

    assert_eq!(
        indoc! {r#"
            [{"file":"ndjson1","id":1,"sub_id":11,"val":"both"},{"file":"ndjson2","id2":1,"sub_id":11,"val":"both"}]
            [{"file":"ndjson1","id":2,"sub_id":21,"val":"both/multi 1"},{"file":"ndjson2","id2":2,"sub_id":21,"val":"both/multi 1"}]
            [{"file":"ndjson1","id":2,"sub_id":22,"val":"both/multi 1"},{"file":"ndjson2","id2":2,"sub_id":21,"val":"both/multi 1"}]
            [{"file":"ndjson1","id":3,"sub_id":31,"val":"both/multi 2"},{"file":"ndjson2","id2":3,"sub_id":31,"val":"both/multi 2"}]
            [{"file":"ndjson1","id":3,"sub_id":31,"val":"both/multi 2"},{"file":"ndjson2","id2":3,"sub_id":32,"val":"both/multi 2"}]
            [{"file":"ndjson1","id":6,"sub_id":62,"val":"both/multi"},{"file":"ndjson2","id2":6,"sub_id":62,"val":"both/multi"}]
            [{"file":"ndjson1","id":6,"sub_id":62,"val":"both/multi"},{"file":"ndjson2","id2":6,"sub_id":61,"val":"both/multi"}]
            [{"file":"ndjson1","id":6,"sub_id":61,"val":"both/multi"},{"file":"ndjson2","id2":6,"sub_id":62,"val":"both/multi"}]
            [{"file":"ndjson1","id":6,"sub_id":61,"val":"both/multi"},{"file":"ndjson2","id2":6,"sub_id":61,"val":"both/multi"}]
            [{"file":"ndjson1","id":11,"sub_id":111,"val":"both"},{"file":"ndjson2","id2":11,"sub_id":111,"val":"both"}]
            [{"file":"ndjson1","id":12,"sub_id":121,"val":"both/multi 1"},{"file":"ndjson2","id2":12,"sub_id":121,"val":"both/multi 1"}]
            [{"file":"ndjson1","id":12,"sub_id":122,"val":"both/multi 1"},{"file":"ndjson2","id2":12,"sub_id":121,"val":"both/multi 1"}]
            [{"file":"ndjson1","id":13,"sub_id":131,"val":"both/multi 2"},{"file":"ndjson2","id2":13,"sub_id":131,"val":"both/multi 2"}]
            [{"file":"ndjson1","id":13,"sub_id":131,"val":"both/multi 2"},{"file":"ndjson2","id2":13,"sub_id":132,"val":"both/multi 2"}]
            [{"file":"ndjson1","id":16,"sub_id":162,"val":"both/multi"},{"file":"ndjson2","id2":16,"sub_id":162,"val":"both/multi"}]
            [{"file":"ndjson1","id":16,"sub_id":162,"val":"both/multi"},{"file":"ndjson2","id2":16,"sub_id":161,"val":"both/multi"}]
            [{"file":"ndjson1","id":16,"sub_id":161,"val":"both/multi"},{"file":"ndjson2","id2":16,"sub_id":162,"val":"both/multi"}]
            [{"file":"ndjson1","id":16,"sub_id":161,"val":"both/multi"},{"file":"ndjson2","id2":16,"sub_id":161,"val":"both/multi"}]
        "#},
        str::from_utf8(&buf).unwrap()
    );
}
#[test]
fn test_join_without_key() {
    let mut f1 = tempfile::tempfile().unwrap();
    write!(f1, "{}", concat!(r#"{"id":1}"#, "\n", r#"{"id":2}"#)).unwrap();
    f1.seek(io::SeekFrom::Start(0)).unwrap();

    let mut f2 = tempfile::tempfile().unwrap();
    write!(f2, "{}", concat!(r#"{"id":11}"#, "\n", r#"{"id":22}"#)).unwrap();
    f2.seek(io::SeekFrom::Start(0)).unwrap();

    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    let r = join(
        f1,
        "noid",
        f2,
        "noid",
        fout,
        Opts {
            allow_no_key: false,
            merge: None,
        },
    );

    assert_eq!(
        r.err().unwrap().to_string(),
        r#"Key 'noid' does not exist: {"id":1}"#
    );
}

#[test]
fn test_join_allow_no_key() {
    let mut f1 = tempfile::tempfile().unwrap();
    write!(f1, "{}", concat!(r#"{"id":1}"#, "\n", r#"{"id":2}"#)).unwrap();
    f1.seek(io::SeekFrom::Start(0)).unwrap();

    let mut f2 = tempfile::tempfile().unwrap();
    write!(f2, "{}", concat!(r#"{"id":11}"#, "\n", r#"{"id":22}"#)).unwrap();
    f2.seek(io::SeekFrom::Start(0)).unwrap();

    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    join(
        f1,
        "noid",
        f2,
        "noid",
        fout,
        Opts {
            allow_no_key: true,
            merge: None,
        },
    )
    .unwrap();

    assert_eq!(
        indoc! {r#"
            [{"id":1},{"id":11}]
            [{"id":1},{"id":22}]
            [{"id":2},{"id":11}]
            [{"id":2},{"id":22}]
        "#},
        str::from_utf8(&buf).unwrap()
    );
}
