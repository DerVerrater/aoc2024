
pub fn process_d9p1(input: &str) -> i32 {
    let _ = input
        .chars()
        .map(|letter| letter.to_digit(10).unwrap() ) // crash if the digit isn't a digit. They have to be.
        .enumerate()
        .map(|(idx, ch)| {
            if idx % 2 == 0 {
                // data
            } else {
                // empty
            }
        });
    
    return 0;
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