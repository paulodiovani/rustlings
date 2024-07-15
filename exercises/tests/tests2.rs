// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

use std::ops::Not;

#[derive(PartialEq, Debug)]
struct Foo {
    foo: String,
}

impl Not for Foo {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self { foo: self.foo.chars().rev().collect::<String>() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        let foo = Foo { foo: "hello".to_string() };
        let bar = Foo { foo: "olleh".to_string() };
        assert_eq!(!foo, bar);
    }
}
