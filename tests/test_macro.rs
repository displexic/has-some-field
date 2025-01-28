use has_some_field::HasSomeField;

#[test]
fn test() {
    #[derive(HasSomeField)]
    struct TestStruct {
        a: Option<u8>,
        b: Option<u8>,
    }

    {
        let test_struct = TestStruct {
            a: Some(5),
            b: Some(5),
        };

        assert_eq!(test_struct.some_field_count(), 2);
    }

    {
        let test_struct = TestStruct { a: None, b: None };
        assert!(!test_struct.has_some_field());
    }

    {
        let test_struct = TestStruct {
            a: Some(5),
            b: None,
        };
        assert!(test_struct.has_some_field());
    }
}

#[test]
fn ignore_test() {
    #[derive(HasSomeField)]
    struct TestStruct {
        a: Option<u8>,
        #[allow(dead_code)]
        #[ignore_has_some]
        b: Option<u8>,
    }

    {
        let test_struct = TestStruct {
            a: Some(5),
            b: Some(5),
        };

        assert_eq!(test_struct.some_field_count(), 1);
    }
}
