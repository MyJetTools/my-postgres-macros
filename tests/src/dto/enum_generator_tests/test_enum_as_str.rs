use my_postgres_macros::DbEnumAsString;

#[derive(DbEnumAsString)]
pub enum MyEnum {
    #[enum_case("1")]
    Case1,
    Case2,
    #[enum_case("5")]
    Case5,
    Case6,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let test = MyEnum::Case1;

        assert_eq!(test.to_str(), "1");
        assert!(matches!(MyEnum::from_str("1"), MyEnum::Case1,));

        let test = MyEnum::Case2;

        assert_eq!(test.to_str(), "Case2");
        assert!(matches!(MyEnum::from_str("Case2"), MyEnum::Case2,));

        let test = MyEnum::Case5;

        assert_eq!(test.to_str(), "5");

        assert!(matches!(MyEnum::from_str("5"), MyEnum::Case5,));

        let test = MyEnum::Case6;

        assert_eq!(test.to_str(), "Case6");

        assert!(matches!(MyEnum::from_str("Case6"), MyEnum::Case6));
    }
}
