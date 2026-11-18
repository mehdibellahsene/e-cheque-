//! Tests de [`e_cheque::greeting`].

use e_cheque::greeting;

#[test]
fn greeting_returns_hello_world() {
    // Arrange
    let expected = "Hello, world!";

    // Act
    let actual = greeting();

    // Assert
    assert_eq!(actual, expected);
}
