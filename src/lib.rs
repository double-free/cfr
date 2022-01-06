mod blotto;

#[cfg(test)]
mod tests {
    use super::blotto;

    #[test]
    fn test_allocation() {
        let alloc = blotto::Allocation::new(3, 5);
        assert_eq!(alloc.soldiers, vec![5, 0, 0]);
    }

    #[test]
    fn test_compare() {
        let mut alloc1 = blotto::Allocation::new(3, 5);
        let mut alloc2 = blotto::Allocation::new(3, 5);
        assert_eq!(alloc1.compare(&alloc2), blotto::Result::Draw);

        alloc1.soldiers[0] = 2;
        alloc1.soldiers[1] = 2;
        alloc1.soldiers[2] = 1;
        // alloc1 lose battle 0, win 1 and 2
        assert_eq!(alloc1.compare(&alloc2), blotto::Result::Win);

        alloc2.soldiers[0] = 1;
        alloc2.soldiers[1] = 2;
        alloc2.soldiers[2] = 2;
        // alloc1 win battle 0, draw 1 and lose 2
        assert_eq!(alloc1.compare(&alloc2), blotto::Result::Draw);

        alloc2.soldiers[0] = 3;
        alloc2.soldiers[1] = 0;
        alloc2.soldiers[2] = 2;
        // alloc1 lose battle 0, win 1 and lose 2
        assert_eq!(alloc1.compare(&alloc2), blotto::Result::Lose);
    }
}
