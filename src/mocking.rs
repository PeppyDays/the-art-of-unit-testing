use mockall::automock;

#[automock]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
    fn bar(&self, x: u32, y: u32) -> u32;
}

struct NonClonable();

#[automock]
trait SecondTrait {
    fn foo(&self) -> NonClonable;
}

trait ThirdTrait {
    fn foo(&self, x: u32) -> u32;
}

fn call_foo_with_four(x: &dyn MyTrait) -> u32 {
    x.foo(4)
}

fn call_bar_with_four(x: &dyn MyTrait) -> u32 {
    x.bar(4, 4)
}

#[cfg(test)]
mod tests {
    use mockall::predicate;

    use super::*;

    mockall::mock! {
        ThirdTrait {}
        impl ThirdTrait for ThirdTrait {
            fn foo(&self, x: u32) -> u32;
        }
    }

    #[test]
    fn mock_without_annotation() {
        let mut mocked = MockThirdTrait::new();
        mocked.expect_foo().returning(|x| x + 1);

        assert_eq!(mocked.foo(4), 5);
    }

    #[test]
    fn simple_mock() {
        let mut mocked = MockMyTrait::new();
        mocked
            .expect_foo()
            .with(predicate::eq(4))
            // for mock, by the way, is there any way to use this for AAA pattern?
            .times(1)
            // for stub
            .returning(|x| x + 1);

        assert_eq!(call_foo_with_four(&mocked), 5);
    }

    #[test]
    fn simple_two_arguments_mock() {
        let mut mocked = MockMyTrait::new();
        mocked
            .expect_bar()
            .with(predicate::eq(4), predicate::eq(4))
            .returning(|x, y| x + y);
        mocked.expect_foo().return_const(42u32);

        assert_eq!(call_foo_with_four(&mocked), 42);
        assert_eq!(call_bar_with_four(&mocked), 8);
    }

    #[test]
    fn non_clonable_can_be_returned_as_once() {
        let r = NonClonable();
        let mut mocked = MockSecondTrait::new();
        mocked.expect_foo().return_once(move || r);
    }

    #[test]
    fn multiple_calls() {
        let mut mocked = MockMyTrait::new();
        mocked
            .expect_foo()
            .with(predicate::eq(5))
            .return_const(10u32);
        mocked
            .expect_foo()
            .with(predicate::eq(6))
            .return_const(12u32);
        mocked.expect_foo().return_const(0u32);

        assert_eq!(mocked.foo(4), 0);
        assert_eq!(mocked.foo(6), 12);
        assert_eq!(mocked.foo(5), 10);
        assert_eq!(mocked.foo(5), 10);
    }
}
