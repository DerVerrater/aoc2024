pub fn process_d3p1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut scan_location = input;
    while let Ok(keyword_start) = try_find_mul(scan_location) {
        // Get remaining input and digits, or else advance slice by 1 and try again.
        let Ok((remainder, first_digits)) = try_collect_digits(keyword_start) else {
            // TODO: Advance by 4 to skip the whole "mul(" text, instead of scanning "ul(_", "l(__", and "(___"
            scan_location = &scan_location[1..];
            continue;
        };
        // consume the following comma, or else advance slice
        let Ok(remainder) = try_consume_comma(remainder) else {
            scan_location = &scan_location[1..];
            continue;
        };
        // get the *next* input digits, or else advance slice
        let Ok((remainder, second_digits)) = try_collect_digits(remainder) else {
            scan_location = &scan_location[1..];
            continue;
        };
        // get the closing parenthesis, or else advance slice
        let Ok(remainder) = try_consume_closeparen(remainder) else {
            scan_location = &scan_location[1..];
            continue;
        };

        // parsing has succeeded in finding a mul(x,y) instruction!

        // 1: Advance scan location forward to the remaining text
        scan_location = remainder;
        // 2: Multiply & accumulate the numbers
        sum += first_digits * second_digits;
    }
    return sum;
}

fn try_find_mul(input: &str) -> Result<&str, ParseError> {
    if let Some(idx) = input.find("mul(") {
        return Ok(&input[idx + 4..]);
    } else {
        return Err(ParseError::EndOfText);
    }
}

fn try_collect_digits(input: &str) -> Result<(&str, i32), ParseError> {
    let mut char_iter = input.chars();
    let mut end_idx: usize = 0;

    while let Some(c) = char_iter.next() {
        if c.is_ascii_digit() {
            end_idx += 1
        } else {
            break;
        };
    }

    // An ending index > 0 means we found some digits.
    if end_idx > 0 {
        // characters must be digits. The unsuccessful parse() path must be unreachable.
        let number = &input[..end_idx]
            .parse::<i32>()
            .expect("String-to-integer conversion failed. This should be unreachable!");
        Ok((&input[end_idx..], *number))
    } else {
        // `end_idx == 0` means there were no numbers. Return an error indicating this.
        Err(ParseError::NoNumber)
    }
}

/*
Tries to find a comma at the beginning of the input string.

On success, returns the input with the comma sliced off for further processing.
On failure, returns a ParseError::NoComma
*/
fn try_consume_comma(input: &str) -> Result<&str, ParseError> {
    if let Some(c) = input.chars().next() {
        if c == ',' {
            return Ok(&input[1..]);
        } else {
            return Err(ParseError::NoComma);
        }
    } else {
        return Err(ParseError::EndOfText);
    }
}

fn try_consume_closeparen(input: &str) -> Result<&str, ParseError> {
    if let Some(c) = input.chars().next() {
        if c == ')' {
            // Indicate successful parse
            return Ok(&input[1..]);
        } else {
            // indicate there's no closing parenthesis
            return Err(ParseError::MissingCloseParen);
        }
    } else {
        // Indicate there was no character at all.
        return Err(ParseError::EndOfText);
    }
}

#[derive(Debug, PartialEq)]
enum ParseError {
    // MissingOpenParen isn't here because the match is part of the "mul(" pattern
    MissingCloseParen,
    NoNumber,
    NoComma,
    EndOfText,
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_TEXT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn run_part1_example() {
        let result = process_d3p1(SAMPLE_TEXT);
        eprintln!("Got result: {result}");
        assert_eq!(result, 161);
    }

    #[test]
    fn find_numbers_perfect() {
        let input = "12345";
        let expected = ("", 12345);
        let result = try_collect_digits(input);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn find_numbers_trailing_text() {
        let input = "12345six seven eight";
        let expected = ("six seven eight", 12345);
        let result = try_collect_digits(input);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn find_numbers_leading_text() {
        let input = "this will fail 1337 times";
        let expected = ParseError::NoNumber;
        let result = try_collect_digits(input);
        assert_eq!(result.unwrap_err(), expected);
    }
}
