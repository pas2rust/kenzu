/*#![cfg(all(
    test,
    feature = "builder",
    not(feature = "debugger"),
    not(feature = "tracing")
))]*/
use kenzu::Builder;
#[derive(Builder, PartialEq)]
pub struct User {
    pub id: String,
    #[set(value = "name")]
    pub name: String,
    pub password: String,
    #[build(
        pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
        err = "err"
    )]
    #[set(value = "email@example.com")]
    pub email: String,
    #[range(min = 18, max = 99, err = "Age must be between 18 and 99")]
    #[set(value = 18)]
    pub age: u8,
    pub gender: String,
    friend: Friend,
}

#[derive(Builder, PartialEq)]
pub struct Friend {
    #[set(value = "default name")]
    name: String,
}

#[test]
fn builder() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(25)
        .build()
        .unwrap();

    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 25);
    assert_eq!(user.friend.name, "default name");
}

#[test]
fn builder_invalid() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("iasjdiasjdasijii")
        .age(25)
        .build();
    assert_eq!(user.err().unwrap(), "err");
}

#[test]
fn builder_set_value() {
    let user = User::new();
    assert_eq!(user.id, "");
    assert_eq!(user.name, "name");
    assert_eq!(user.email, "email@example.com");
    assert_eq!(user.age, 18);
    assert_eq!(user.gender, "");
    assert_eq!(user.friend.name, "default name");
}
