#[cfg(test)]
mod some_structure_tests {
    // use super::super::some_structure::*;
    // we adjust the path to:
    use super::super::*;

    // #[test]
    // fn fail_it() {
    //     assert_eq!(1, 3);
    // }

    #[test]
    fn double_test() {
        let sut = SomeStructure { value: 2 };
        assert_eq!(sut.double_value(), 4);
    }

    #[test]
    fn module_private_test() {
        module_private_function();
        assert!(true)
    }
}
