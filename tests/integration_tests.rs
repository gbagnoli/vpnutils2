use anyhow::Result;

static PASSWORD: &str = "supersafe";

#[test]
fn test_create() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let db_path = dir.path().join("database.db");
    let db_str = db_path.into_os_string().into_string().unwrap();
    let db = vpnutils::Database::create(&db_str, PASSWORD.to_string())?;

    assert!(vpnutils::Database::create(&db_str, PASSWORD.to_string()).is_err());

    db.connect()?;
    db.save()?;

    let new = vpnutils::Database::open(&db_str, PASSWORD.to_string())?;
    assert_eq!(db_str, db.path());
    assert_eq!(db.path(), new.path());
    Ok(())
}
