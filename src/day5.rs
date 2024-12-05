use std::collections::HashSet;

use itertools::Itertools;

pub fn process_d5p1(input: &str) -> i32 {
    todo!();
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Rule {
    left: i32,
    right: i32,
}

impl Rule {
    const fn new(left: i32, right: i32) -> Self {
        Self { left, right }
    }
}

/*
I'm lazy, and this selection is just a set intersection. Use HashSet::intersection()
 */
fn select_active_rules(rules: HashSet<Rule>, number_sequence: Vec<i32>) -> HashSet<Rule> {
    // Numbers in the input determine the rules to select. Generate Rule pairs
    // to intersect with the primary rule set.
    let pairs: Vec<Rule> = number_sequence
        .into_iter()
        .combinations(2)
        .map(|combos| Rule::new(*combos.get(0).unwrap(), *combos.get(1).unwrap()))
        .collect();

    let set_pairs: HashSet<Rule> = HashSet::from_iter(pairs.into_iter());
    let intersection: HashSet<Rule> = rules
        .intersection(&set_pairs)
        .map(|item| item.clone())
        .collect();
    return intersection;
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    const ALL_EXAMPLE_RULES: [Rule; 20] = [
        Rule::new(97, 13),
        Rule::new(97, 61),
        Rule::new(97, 47),
        Rule::new(75, 29),
        Rule::new(61, 13),
        Rule::new(75, 53),
        Rule::new(29, 13),
        Rule::new(97, 29),
        Rule::new(53, 29),
        Rule::new(61, 53),
        Rule::new(97, 53),
        Rule::new(61, 29),
        Rule::new(47, 13),
        Rule::new(75, 47),
        Rule::new(97, 75),
        Rule::new(47, 61),
        Rule::new(75, 61),
        Rule::new(47, 29),
        Rule::new(75, 13),
        Rule::new(53, 13),
    ];

    #[test]
    fn run_part1_example() {
        let expected = 143;
        let result = process_d5p1(SAMPLE_TEXT);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rule_selector() {
        let expected: HashSet<Rule> = HashSet::from_iter(
            vec![
                Rule::new(75, 47),
                Rule::new(75, 61),
                Rule::new(75, 53),
                Rule::new(75, 29),
                Rule::new(47, 61),
                Rule::new(47, 29),
                Rule::new(61, 53),
                Rule::new(61, 29),
                Rule::new(53, 29),
            ]
            .into_iter(),
        );
        let result = select_active_rules(ALL_EXAMPLE_RULES.into(), vec![75, 47, 61, 53, 29]);
        assert_eq!(result, expected);
    }
}
