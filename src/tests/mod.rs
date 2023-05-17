use std::error::Error;

#[test]
fn test_metadata_root() -> Result<(), Box<dyn Error>> {
    if !std::fs::metadata("/tmp").map(|m| m.is_dir()).unwrap_or(false) {
        return Err(format!("Invalid directory: {}", "/tmp").into());
    }
    Ok(())

}