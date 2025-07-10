/*#![cfg(all(
    test,
    feature = "mutating_builder",
    not(feature = "debugger"),
    not(feature = "tracing")
))]*/

use kenzu::M_Builder;

#[derive(M_Builder, PartialEq, Default, Clone, Debug)]
pub struct User<'a> {
    #[set(value = "123e4567-e89b-12d3-a456-426614174000")]
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
    pub fix: &'a str,
    #[set(value = 18)]
    pub age: u8,
    #[set(skip)]
    pub code: u8,
    pub def: String,
}

#[test]
fn mutating_builder() {
    let mut user = User::new();

    user.id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(18)
        .fix("fix")
        .def("def")
        .build()
        .unwrap();

    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 18);
    assert_eq!(user.fix, "fix");
}

#[test]
fn builder_set_value() {
    let user = User::new();

    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "name");
    assert_eq!(user.email, "email@example.com");
    assert_eq!(user.age, 18);
    assert_eq!(user.password, "");
    assert_eq!(user.fix, "");
}

#[test]
fn builder_invalid() {
    let mut user = User::new();
    let user = user
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("iasjdiasjdasijii")
        .age(25)
        .build();

    assert_eq!(user.err().unwrap(), "err");
}
