
pub fn process_d9p1(input: &str) -> i32 {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = "2333133121414131402";

    #[test]
    fn run_part1_example(){
        assert_eq!(1928, process_d9p1(SAMPLE_TEXT))
    }
}