use crate::print_results;
use itertools::Itertools;
use simd_json::{borrowed::Value, prelude::*, to_borrowed_value, BorrowedValue};
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/12");
    let mut input_bytes = input.bytes().collect_vec();
    let parsed = to_borrowed_value(&mut input_bytes).expect("failed to parse input");
    let pt1 = find_numbers(&parsed).iter().sum::<i64>();
    let pt2 = find_non_red_numbers(&parsed).iter().sum::<i64>();
    print_results(2015, 12, pt1, pt2, Some(start));
}

fn find_numbers(val: &Value) -> Vec<i64> {
    match val {
        Value::Array(xs) => xs.iter().flat_map(find_numbers).collect(),
        Value::Object(xs) => xs.values().flat_map(find_numbers).collect(),
        Value::Static(v) => v.as_i64().map(|x| vec![x]).unwrap_or_default(),
        _ => vec![],
    }
}

fn find_non_red_numbers(val: &Value) -> Vec<i64> {
    match val {
        Value::Array(xs) => xs.iter().flat_map(find_non_red_numbers).collect(),
        Value::Object(xs) if !xs.values().contains(&BorrowedValue::from("red")) => {
            xs.values().flat_map(find_non_red_numbers).collect()
        }
        Value::Static(v) => v.as_i64().map(|x| vec![x]).unwrap_or_default(),
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(f: fn(&Value) -> Vec<i64>, input: &str, expected_value: i64) {
        let mut input_bytes = input.bytes().collect_vec();
        let parsed = to_borrowed_value(&mut input_bytes).expect("failed to parse input");
        assert_eq!(f(&parsed).iter().sum::<i64>(), expected_value);
    }

    #[test]
    fn test_find_numbers() {
        check(find_numbers, r#"[1,2,3]"#, 6);
        check(find_numbers, r#"{"a":2,"b":4}"#, 6);
        check(find_numbers, r#"[[[3]]]"#, 3);
        check(find_numbers, r#"{"a":{"b":4},"c":-1}"#, 3);
        check(find_numbers, r#"{"a":[-1,1]}"#, 0);
        check(find_numbers, r#"[-1,{"a":1}]"#, 0);
        check(find_numbers, r#"[]"#, 0);
        check(find_numbers, r#"{}"#, 0);
    }

    #[test]
    fn test_find_non_red_numbers() {
        check(find_non_red_numbers, r#"[1,2,3]"#, 6);
        check(find_non_red_numbers, r#"[1,{"c":"red","b":2},3]"#, 4);
        check(
            find_non_red_numbers,
            r#"{"d":"red","e":[1,2,3,4],"f":5}"#,
            0,
        );
        check(find_non_red_numbers, r#"[1,"red",5]"#, 6);
    }
}
