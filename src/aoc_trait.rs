pub trait AocDay {
    fn num(&self) -> usize;
    fn part_1(&self, s: String) -> i64;
    fn part_2(&self, s: String) -> i64;
    fn test_part_1(&self) -> (i64, bool);
    fn test_part_2(&self) -> (i64, bool);
}
