use io::JagFile;

#[test]
fn test_hash_gnomeball_buttons() {
    assert_eq!(22834782, JagFile::hash("gnomeball_buttons.dat"));
}

#[test]
fn test_hash_headicons() {
    assert_eq!(-288954319, JagFile::hash("headicons.dat"));
}

#[test]
fn test_hash_hitmarks() {
    assert_eq!(-1502153170, JagFile::hash("hitmarks.dat"));
}
