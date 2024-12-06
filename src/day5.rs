use std::collections::HashSet;

use itertools::Itertools;

pub fn process_d5p1(input: &str) -> i32 {
}

fn check_digits(rules: &RuleSet, row: &Vec<i32>) -> bool {
    // 2. for each number...
    for (idx, num) in row.iter().enumerate() {
        // 3. Get rules to enforce
        let to_enforce = select_partial_rules(rules, *num);
        for rule in to_enforce {
            // 4. Scan left/right to see if the other value is present.
            if check_rule(&rule, &row, idx) {
                continue;
            } else {
                return false;
            }
        }
    }
    return true;
}

/*
Given a rule, row, and current index, check that the rule is satisfied for
the number at that index.
 */
fn check_rule(rule: &Rule, row: &Vec<i32>, idx: usize) -> bool {
    // The number whose position we're validating
    let number = row.get(idx).unwrap();

    // Get the other value from the Rule, and search towards the end in that direction
    if *number == rule.left {
        // We're on left number? Scan right for right number.
        for i in idx..row.len() {
            let v = row.get(i).unwrap();
            if *v == rule.right {
                return true;
            }
        }
    } else if *number == rule.right {
        // We're on right number/ Scan left for left number.
        for i in (0..idx).rev() {
            let v = row.get(i).unwrap();
            if *v == rule.left {
                return true;
            }
        }
    } else {
        unreachable!("Sanity check. This should be unreachable, so panic if we get here.")
    };
    false
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Rule {
    left: i32,
    right: i32,
}

impl Rule {
    const fn new(left: i32, right: i32) -> Self {
        Self { left, right }
    }

    const fn contains(&self, num: i32) -> bool {
        self.left == num || self.right == num
    }
}

type RuleSet = HashSet<Rule>;

/*
I'm lazy, and this selection is just a set intersection. Use HashSet::intersection()
 */
fn select_active_rules(rules: &RuleSet, number_sequence: &Vec<i32>) -> RuleSet {
    // Numbers in the input determine the rules to select. Generate Rule pairs
    // to intersect with the primary rule set.
    // Order cannot be enforced because the input number sequence might be out of order!
    // Use `permitations(2)` instead of `combinations(2)`
    let pairs: Vec<Rule> = number_sequence
        .into_iter()
        .permutations(2)
        .map(|combos| Rule::new(**combos.get(0).unwrap(), **combos.get(1).unwrap()))
        .collect();

    let set_pairs: RuleSet = HashSet::from_iter(pairs.into_iter());
    let intersection: RuleSet = rules
        .intersection(&set_pairs)
        .map(|item| item.clone())
        .collect();
    return intersection;
}

/*
Filter a RuleSet to include only Rules which have `number` as one of their components.
 */
fn select_partial_rules<'rulelife>(
    rules: &'rulelife RuleSet,
    number: i32,
) -> impl Iterator<Item = Rule> + 'rulelife {
    rules
        .iter()
        .filter(move |rule| rule.contains(number))
        .map(|rule_ref| rule_ref.clone())
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

    const ALL_EXAMPLE_RULES: [Rule; 21] = [
        Rule::new(47, 53),
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
        let expected: RuleSet = HashSet::from_iter(
            vec![
                Rule::new(75, 47),
                Rule::new(75, 61),
                Rule::new(75, 53),
                Rule::new(75, 29),
                Rule::new(47, 61),
                Rule::new(47, 53),
                Rule::new(47, 29),
                Rule::new(61, 53),
                Rule::new(61, 29),
                Rule::new(53, 29),
            ]
            .into_iter(),
        );
        let result = select_active_rules(&ALL_EXAMPLE_RULES.into(), &vec![75, 47, 61, 53, 29]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rule_selector_outoforder() {
        let expected: RuleSet = HashSet::from_iter(
            vec![
                /*
                This first one is in the primary list, but the sequence will look for `75, 97`
                 */
                Rule::new(97, 75),
                Rule::new(75, 47),
                Rule::new(75, 61),
                Rule::new(75, 53),
                Rule::new(97, 47),
                Rule::new(97, 61),
                Rule::new(97, 53),
                Rule::new(47, 61),
                Rule::new(47, 53),
                Rule::new(61, 53),
            ]
            .into_iter(),
        );
        let result = select_active_rules(&ALL_EXAMPLE_RULES.into(), &vec![75, 97, 47, 61, 53]);
        assert_eq!(result, expected);
    }

    #[test]
    fn partial_rule_selector() {
        let expected: RuleSet = HashSet::from_iter(vec![
            Rule::new(97, 13),
            Rule::new(97, 61),
            Rule::new(97, 47),
            Rule::new(97, 29),
            Rule::new(97, 53),
            Rule::new(97, 75),
        ].into_iter());
        let result = HashSet::from_iter(select_partial_rules(&ALL_EXAMPLE_RULES.into(), 97));
        assert_eq!(result, expected);
    }

    #[test]
    fn check_rule_checker_idx0_rightscan() {
        let rules = vec![
            Rule::new(75, 47),
            Rule::new(75, 61),
            Rule::new(75, 53),
            Rule::new(75, 29),
        ];
        let input_row = vec![75, 47, 61, 53, 29];
        for rule in rules {
            assert!(check_rule(&rule, &input_row, 0));
        }
    }

    #[test]
    fn check_rule_checker_idx1_rightscan() {
        let rules = vec![Rule::new(75, 47), Rule::new(47, 61), Rule::new(47, 29)];
        let input_row = vec![75, 47, 61, 53, 29];
        for rule in rules {
            assert!(check_rule(&rule, &input_row, 1));
        }
    }

    #[test]
    fn check_rule_checker_idx_end_leftscan() {
        let rules = vec![Rule::new(75, 29), Rule::new(61, 29), Rule::new(53, 29)];
        let input_row = vec![75, 47, 61, 53, 29];
        for rule in rules {
            assert!(check_rule(&rule, &input_row, 4));
        }
    }

    #[test]
    fn check_rule_checker_idx_end_m1_leftscan() {
        let rules = vec![Rule::new(75, 53), Rule::new(61, 53), Rule::new(53, 29)];
        let input_row = vec![75, 47, 61, 53, 29];
        for rule in rules {
            assert!(check_rule(&rule, &input_row, 3));
        }
    }

    #[test]
    fn check_check_digits_good() {
        let sequence = vec![75, 47, 61, 53, 29]; // Known good sequence (from example text)
        let to_enforce = select_active_rules(&ALL_EXAMPLE_RULES.into(), &sequence);
        assert!(check_digits(&to_enforce, &sequence));
    }

    #[test]
    fn check_check_digits_bad() {
        let sequence = vec![75, 97, 47, 61, 53]; // Known BAD sequence (from example text)
        let to_enforce = select_active_rules(&ALL_EXAMPLE_RULES.into(), &sequence);
        assert!(!check_digits(&to_enforce, &sequence));
    }
}
