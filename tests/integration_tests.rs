use anyhow::Result;

static PASSWORD: &str = "supersafe";

#[test]
fn test_database() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let db_path = dir.path().join("database.db");
    let db_str = db_path.into_os_string().into_string().unwrap();

    // file does not exists
    assert!(vpnutils::Database::open(&db_str, String::from("otherpass")).is_err());

    // I can create, connect, and save back
    let db = vpnutils::Database::create(&db_str, PASSWORD.to_string())?;
    db.connect()?;
    db.save()?;

    // cannot create twice
    assert!(vpnutils::Database::create(&db_str, PASSWORD.to_string()).is_err());

    // password mismatch
    assert!(vpnutils::Database::open(&db_str, String::from("otherpass")).is_err());

    let new = vpnutils::Database::open(&db_str, PASSWORD.to_string())?;
    assert_eq!(db_str, db.path());
    assert_eq!(db.path(), new.path());
    Ok(())
}
