pub mod inverse_captcha;

#[cfg(test)]
mod inverse_captcha_tests {
    #[test]
    fn duplicates_sum() {
        assert!(::inverse_captcha::solve("1122").unwrap() == 3);
    }

    #[test]
    fn non_consecutive_doesnt_sum() {
        assert!(::inverse_captcha::solve("1234").unwrap() == 0);
    }

    #[test]
    fn invalid_input_returns_none() {
        assert!(::inverse_captcha::solve("11-22").is_none());
    }

    #[test]
    fn wraps_around_string() {
        assert!(::inverse_captcha::solve("91212129").unwrap() == 9);
    }
}