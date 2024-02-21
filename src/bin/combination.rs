struct InputForm {
    name: String,
    inquiry: String,
}

fn validate(input: &InputForm) -> bool {
    input.name.len() > 0 && input.inquiry.len() > 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn should_accepct_all_corner_cases(
        #[values("customer", "complainer")] name: String,
        #[values(
            "I'm afraid but could you explain why my luggage didn't get delivered on schedule?",
            "Hey, what's going on my luggage!"
        )]
        inquiry: String,
    ) {
        assert!(validate(&InputForm { name, inquiry }))
    }
}

fn main() {}
