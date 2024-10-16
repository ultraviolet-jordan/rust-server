use io::Isaac;

#[test]
fn test_isaac() {
    let mut isaac = Isaac::new(vec![1, 2, 3, 4]);
    for _ in 0..1000000 {
        isaac.next();
    }
    assert_eq!(-107094133, isaac.next());
}

#[test]
fn test_isaac_zeroed() {
    let mut isaac = Isaac::new(vec![0, 0, 0, 0]);
    for _ in 0..1000000 {
        isaac.next();
    }
    assert_eq!(1536048213, isaac.next());
}
