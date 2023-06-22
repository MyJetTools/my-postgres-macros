use my_postgres_macros::DbEnumAsI16;

#[derive(DbEnumAsI16)]
enum MyEnum {
    #[enum_case(1)]
    Case1,
    Case2,
    #[enum_case(5)]
    Case5,
    Case6,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let test = MyEnum::Case1;

        assert_eq!(test.as_numbered_str(), "1");
        assert_eq!(test.to_i16(), 1);
        assert!(matches!(MyEnum::from_db_value(1), MyEnum::Case1,));

        let test = MyEnum::Case2;

        assert_eq!(test.as_numbered_str(), "2");
        assert_eq!(test.to_i16(), 2);
        assert!(matches!(MyEnum::from_db_value(2), MyEnum::Case2,));

        let test = MyEnum::Case5;

        assert_eq!(test.as_numbered_str(), "5");
        assert_eq!(test.to_i16(), 5);
        assert!(matches!(MyEnum::from_db_value(5), MyEnum::Case5,));

        let test = MyEnum::Case6;

        assert_eq!(test.as_numbered_str(), "6");
        assert_eq!(test.to_i16(), 6);
        assert!(matches!(MyEnum::from_db_value(6), MyEnum::Case6,));
    }
}
