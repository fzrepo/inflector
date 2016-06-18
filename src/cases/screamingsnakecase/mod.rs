use std::ascii::*;
use regex::Regex;

use cases::uppercase::to_upper_case;

/// Converts a `String` to `SCREAMING_SNAKE_CASE` `String`
///
/// #Examples
/// ```
/// use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///
///
/// // screaming_snake_case_foo_bar_as_foo_bar() {
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "FOO_BAR".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///
///
/// // screaming_snake_case_HTTP_Foo_space_bar_as_foo_bar() {
///     let mock_string: String = "HTTP Foo bar".to_string();
///     let expected_string: String = "HTTP_FOO_BAR".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///
///
/// // screaming_snake_case_Foo_space_bar_as_foo_bar() {
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "FOO_BAR".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///
///
/// // screaming_snake_case_Foo_space_Bar_as_foo_bar() {
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "FOO_BAR".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///
///
/// // screaming_snake_case_FooBar_as_foo_bar() {
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "FOO_BAR".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///
///
/// // screaming_snake_case_fooBar_as_foo_bar() {
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "FOO_BAR".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
/// use inflector::cases::screamingsnakecase::to_screaming_snake_case;
///
///
/// // screaming_snake_case_fooBar3_as_foo_bar() {
///     let mock_string: String = "fooBar3".to_string();
///     let expected_string: String = "FOO_BAR_3".to_string();
///     let asserted_string: String = to_screaming_snake_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_screaming_snake_case(non_snake_case_string: String) -> String {
    if non_snake_case_string.contains(' ')
        || non_snake_case_string.contains('_')
        || non_snake_case_string.contains('-') {
        to_snake_from_sentence_or_kebab(non_snake_case_string)
    } else {
        to_snake_from_camel_or_class(non_snake_case_string)
    }
}
    fn to_snake_from_camel_or_class (non_snake_case_string: String) -> String {
        let mut result:String = "".to_string();
        let mut first_character: bool = true;
        for character in non_snake_case_string.chars() {
            if character == character.to_ascii_uppercase() && !first_character {
                result = format!("{}_{}", result, character.to_ascii_uppercase());
            } else {
                result = format!("{}{}", result, character.to_ascii_uppercase());
                first_character = false;
            }
        }
        result
    }

    fn to_snake_from_sentence_or_kebab(non_snake_case_string: String) -> String {
        to_upper_case(non_snake_case_string.replace(" ", "_").replace("-", "_"))
    }

/// Determines of a `String` is `SCREAMING_SNAKE_CASE`
///
/// #Examples
/// ```
/// use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///
///
/// // returns_falsey_value_for_is_screaming_snake_case_when_sentence() {
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
/// use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///
///
/// // returns_falsey_value_for_is_screaming_snake_case_when_kebab() {
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
/// use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///
///
/// // returns_falsey_value_for_is_screaming_snake_case_when_class() {
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
/// use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///
///
/// // returns_falsey_value_for_is_screaming_snake_case_when_title() {
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
/// use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///
///
/// // returns_falsey_value_for_is_screaming_snake_case_when_camel() {
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
/// use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///
///
/// // returns_truthy_value_for_is_screaming_snake_case_when_snake() {
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
/// use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///
///
/// // returns_truthy_value_for_is_screaming_snake_case_when_snake_with_a_number() {
///     let mock_string: String = "FOO_BAR1_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
/// use inflector::cases::screamingsnakecase::is_screaming_snake_case;
///
///
/// // returns_truthy_value_for_is_screaming_snake_case_when_snake_with_a_number_snaked() {
///     let mock_string: String = "FOO_BAR_1_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_screaming_snake_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
pub fn is_screaming_snake_case(test_string: String) -> bool{
    let snake_matcher = Regex::new(r"(?:[^-|^ ]?=^|[_])([A-Z]+)").unwrap();
    if snake_matcher.is_match(test_string.as_ref()){
        return true;
    }
    false
}
