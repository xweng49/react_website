
///test doc comments. Hover over this function for neat instructions.
///```
///let Rene = "a little bitch";
///
///```

 /// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
pub fn test_function() -> i32 {
    10
}


#[cfg(test)]
mod tests {
    use super::test_function;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_test_function() {
        assert_eq!(10, test_function());
    }
}
