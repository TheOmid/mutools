pub mod test_projects;

#[test]
pub fn test_testing() {
    let result = 2 + 2;
    assert_eq!(result, 4);
    assert_ne!(result, 10);
}
