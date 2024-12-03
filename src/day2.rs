use itertools::Itertools;

type Record = Vec<i32>;

fn parse_reports(input: &str) -> impl Iterator<Item = Record> + use<'_> {
    input.split("\n").map(|line| -> Record {
        let parts = line.split(" ");
        let num_iter = parts.map(|number| number.parse::<i32>().expect("Failed to parse a number"));
        return num_iter.collect::<Record>();
    })
}

#[derive(Debug, PartialEq)]
enum SafetyGrade {
    Safe, // report is safe. Count this one!
    // usize is index of level to delete before re-testing
    TooSlow(usize), // report changes by < 1
    TooFast(usize), // report changes by > 3
    Maxima(usize),  // report was going up, and is now going down
    Minima(usize),  // report was going down, and is now going up
}

fn grade_record(record: Record) -> SafetyGrade {
    let mut increasing = false;
    let mut decreasing = false;

    let diffs: Vec<i32> = record
        .into_iter()
        .tuple_windows()
        .map(|(current, next)| next - current)
        .collect();

    for (idx, slope) in diffs.into_iter().enumerate() {
        if slope == 0 {
            // There was no change. Fail TooSlow
            return SafetyGrade::TooSlow(idx);
        } else if slope > 0 {
            // slope increasing. Check if local minima, then slope magnitude
            increasing = true;
            if decreasing {
                // if decreasing was set before, this is a (local) minima. Fail
                return SafetyGrade::Minima(idx);
            } else if slope > 3 {
                // if rising faster than 3, emit failure
                return SafetyGrade::TooFast(idx);
            }
        } else if slope < 0 {
            // slope decreasing. Check if local maxima, then slope magnitude
            decreasing = true;
            if increasing {
                // if increasing was set before, this is a (local) maxima. Fail
                return SafetyGrade::Maxima(idx);
            } else if slope < -3 {
                // if dropping faster than -3, emit failure
                return SafetyGrade::TooFast(idx);
            }
        }
    }

    // no unsafe values were detected. Must be safe
    return SafetyGrade::Safe;
}

pub fn process_d2p1(input: &str) -> i32 {
    parse_reports(input)
        .map(grade_record)
        .fold(0, |count, grade| {
            if grade == SafetyGrade::Safe {
                count + 1
            } else {
                count
            }
        })
}

pub fn process_d2p2(input: &str) -> i32 {
    // Collect the failed items for a second round of processing
    let mut needs_another: Vec<(Record, SafetyGrade)> = Vec::new();

    // Same routine as Part 1: Parse text, grade records, count safe ones
    let passes = parse_reports(input)
        .map(|record| (record.clone(), grade_record(record)))
        .fold(0, |count, (record, grade)| {
            if grade == SafetyGrade::Safe {
                count + 1
            } else {
                needs_another.push((record, grade));
                count
            }
        });

    // Remove the failing index from each record before reprocessing
    let needs_another = needs_another
        .into_iter()
        .map(|(mut record, grade)| {
            let idx = match grade {
                SafetyGrade::Safe => unreachable!(),
                SafetyGrade::TooSlow(idx) => idx,
                SafetyGrade::TooFast(idx) => idx,
                SafetyGrade::Maxima(idx) => idx,
                SafetyGrade::Minima(idx) => idx,
            };
            record.remove(idx);
            record
        })
        .collect::<Vec<Record>>();

    let recovered_passes = needs_another
        .into_iter()
        .map(|record| (record.clone(), grade_record(record)))
        .map(|(record, grade)| {
            println!("Record {:?} was graded {:?}", record, grade);
            grade
        })
        .fold(0, |count, grade| {
            if grade == SafetyGrade::Safe {
                count + 1
            } else {
                count
            }
        });
    println!(
        "Passes: {}\nRecovered: {}\n  Total: {}",
        passes,
        recovered_passes,
        (passes + recovered_passes)
    );
    passes + recovered_passes
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    const SAMPLE_TEXT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    const MORE_EDGE_CASES: &str = "5 8 4 3 1
5 9 6 7 8";

    #[test]
    fn run_part1_example() {
        let result = process_d2p1(SAMPLE_TEXT);
        eprintln!("Got result: {result}");
        assert_eq!(result, 2);
    }

    #[test]
    fn run_part2_example() {
        let result = process_d2p2(SAMPLE_TEXT);
        eprintln!("Got result: {result}");
        assert_eq!(result, 4);
    }

    #[test]
    fn run_part2_edge_cases() {
        /*
        The sequence `5 8 4 3 2` can pass if the '8' is deleted. This is
        currently done correctly.

        The sequence `5 9 6 7 8` can pass if the 9 is deleted. Instead, the
        program finds `5 9`, rejects it as ::TooFast(0) and deletes the 5.
        The second pass then rejects again with a ::Minima(0)
        */
        let result = process_d2p2(MORE_EDGE_CASES);
        eprintln!("Got result: {result}");
        assert_eq!(result, 2);
    }
}
