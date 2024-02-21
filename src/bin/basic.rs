#[derive(Debug, Eq, PartialEq)]
enum Year {
    // 平年を表す
    Common,
    // 閏年を表す
    Leap,
}

fn year_kind(year: u32) -> Year {
    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        Year::Leap
    } else {
        Year::Common
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{year_kind, Year};

    #[test]
    #[ignore]
    fn test_year_kind_normaly() {
        let expectations = vec![
            (2000, Year::Leap),
            (2024, Year::Leap),
            (2025, Year::Common),
            (2100, Year::Common),
        ];
        for (year, kind) in expectations {
            assert_eq!(year_kind(year), kind);
        }
    }

    #[rstest]
    #[test]
    #[case(2000, Year::Leap)]
    #[case(2024, Year::Leap)]
    #[case(2025, Year::Common)]
    #[case(2100, Year::Common)]
    #[ignore]
    fn test_year_kind(#[case] year: u32, #[case] kind: Year) {
        assert_eq!(year_kind(year), kind);
    }

    #[rstest]
    #[test]
    #[case::divided_by_100_and_400(2000, Year::Leap)]
    #[case::simply_divided_by_4(2024, Year::Leap)]
    #[case::not_divided_by_4(2025, Year::Common)]
    #[case::not_divided_by_400_and_divided_by_100(2100, Year::Common)]
    fn test_year_kind_named(#[case] year: u32, #[case] kind: Year) {
        assert_eq!(year_kind(year), kind);
    }
}

fn main() {
    println!("Hello, world!");
}
