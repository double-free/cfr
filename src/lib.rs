mod blotto;

#[cfg(test)]
mod tests {
    use super::blotto;

    #[test]
    fn test_allocation() {
        let alloc = blotto::Allocation::new(3, 5);
        assert_eq!(alloc.soldiers, vec![5, 0, 0]);
    }
}
