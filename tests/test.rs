mod test1 {
    use zz_study::sum;
    //
    #[test]
    fn test_01() {
        assert_eq!(sum(1, 3), 4);
        assert_eq!(sum(2u8, 4u8), 6u8);
        assert_eq!(sum(3i64, 5i64), 8i64);
    }
}
