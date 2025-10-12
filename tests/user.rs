use kenzu::Builder;

#[derive(Builder, Debug)]
pub struct User {
    #[opt(default = "123e4567-e89b-12d3-a456-426614174000")]
    id: String,
    #[opt(default = "John Doe")]
    name: String,
    #[opt(default = "password123")]
    password: String,
    #[opt(
        pattern = r#"(?:[a-z0-9!#$%&'*+\x2f=?^_`\x7b-\x7d~\x2d]+(?:\.[a-z0-9!#$%&'*+\x2f=?^_`\x7b-\x7d~\x2d]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9\x2d]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9\x2d]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9\x2d]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#,
        err = "My custom error",
        default = "johndoe@example.com"
    )]
    email: String,
    #[opt(
        min = 18,
        err_min = "My custom err_min",
        max = 125,
        err_max = "My custom err_max",
        default = 18
    )]
    age: u8,
    #[opt(min = 0.0, max = 1.0, default = 0.0)]
    rating: f32,
    #[opt(default=vec![
        Friend::new().name(FriendName::new("Friend name").expect("")), 
        Friend::new().name(FriendName::new("Friend name").expect(""))
    ])]
    friends: Vec<Friend>,
    best_frined: Friend,
}

#[derive(Builder, Debug)]
pub struct Friend {
    #[opt(default = "Friend name")]
    name: String,
}

#[test]
fn invalid_email() {
    let res = UserEmail::new("not-an-email");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "My custom error");
}

#[test]
fn age_too_small() {
    let res = UserAge::new(10);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "My custom err_min");
}

#[test]
fn age_too_large() {
    let res = UserAge::new(130);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "My custom err_max");
}
#[test]
fn rating_too_small() {
    let res = UserRating::new(-0.5);
    assert!(res.is_err());
}

#[test]
fn rating_too_large() {
    let res = UserRating::new(1.5);
    assert!(res.is_err());
}

#[test]
fn valid_values() -> Result<(), String> {
    let age = UserAge::new(30)?;
    let rating = UserRating::new(0.5)?;
    assert_eq!(age.0, 30);
    assert_eq!(rating.0, 0.5);
    Ok(())
}

#[test]
fn default_user() {
    let user = User::new();
    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 18);
    assert_eq!(user.rating, 0.0);
    println!("{user:#?}");
}

#[test]
fn dynamic_user() -> Result<(), String> {
    let user = User::new()
        .id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .name(UserName::new("John Doe")?)
        .password(UserPassword::new("password123")?)
        .age(UserAge::new(18)?)
        .rating(UserRating::new(0.77)?);

    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 18);
    assert_eq!(user.rating, 0.77);
    println!("{user:#?}");
    Ok(())
}

#[test]
fn mut_dynamic_user() -> Result<(), String> {
    let mut user = User::new();
    user.mut_id(UserId::new("123e4567-e89b-12d3-a456-426614174000")?)
        .mut_name(UserName::new("John Doe")?)
        .mut_password(UserPassword::new("password123")?)
        .mut_age(UserAge::new(18)?)
        .mut_rating(UserRating::new(0.77)?);

    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 18);
    assert_eq!(user.rating, 0.77);
    println!("{user:#?}");
    Ok(())
}

#[test]
fn invalid_emails_many() {
    let invalids = [
        "",
        "plainaddress",
        "missing-at-sign.com",
        "@missing-local.org",
        "local@.com",
        "local@com",
        "local@domain..com",
        "local@domain,com",
        "local@domain#com",
        "not-an-email",
    ];

    for s in invalids {
        let res = UserEmail::new(s);
        assert!(res.is_err(), "email `{}` should be invalid", s);
        assert_eq!(res.unwrap_err(), "My custom error");
    }
}

#[test]
fn valid_emails_many() -> Result<(), String> {
    let valids = [
        "simple@example.com",
        "very.common@example.com",
        "disposable.style.email.with+symbol@example.com",
        "other.email-with-dash@example.com",
        "x@x.au",
        "user.name+tag+sorting@example.com",
        "customer/department=shipping@example.com",
        "john.doe@example.co.uk",
        "johndoe@example.com",
    ];

    for s in valids {
        let res = UserEmail::new(s)?;
        assert_eq!(res.0, s);
    }
    Ok(())
}

#[test]
fn age_too_small_and_too_large_again() {
    let too_small = UserAge::new(0);
    assert!(too_small.is_err());
    let too_small2 = UserAge::new(17);
    assert!(too_small2.is_err());
    assert_eq!(too_small2.unwrap_err(), "My custom err_min");

    let too_large = UserAge::new(126);
    assert!(too_large.is_err());
    assert_eq!(too_large.unwrap_err(), "My custom err_max");
}

#[test]
fn age_bounds_are_inclusive() -> Result<(), String> {
    let min_ok = UserAge::new(18)?;
    let max_ok = UserAge::new(125)?;
    assert_eq!(min_ok.0, 18);
    assert_eq!(max_ok.0, 125);
    Ok(())
}

#[test]
fn rating_out_of_range_errors() {
    assert!(UserRating::new(-0.0001f32).is_err());
    assert!(UserRating::new(-1.0f32).is_err());
    assert!(UserRating::new(1.0001f32).is_err());
    assert!(UserRating::new(2.0f32).is_err());
}

#[test]
fn rating_bounds_are_inclusive() -> Result<(), String> {
    let r0 = UserRating::new(0.0)?;
    let r1 = UserRating::new(1.0)?;
    let mid = UserRating::new(0.5)?;
    assert_eq!(r0.0, 0.0);
    assert_eq!(r1.0, 1.0);
    assert_eq!(mid.0, 0.5);
    Ok(())
}

#[test]
fn rating_precision_examples() -> Result<(), String> {
    let near_zero = UserRating::new(0.000_001)?;
    let near_one = UserRating::new(0.999_999)?;
    assert!(near_zero.0 >= 0.0 && near_zero.0 <= 1.0);
    assert!(near_one.0 >= 0.0 && near_one.0 <= 1.0);
    Ok(())
}

#[test]
fn default_user_checks() {
    let user = User::new();
    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 18);
    assert_eq!(user.rating, 0.0);
    assert_eq!(user.friends.iter().len(), 2);
    println!("friends: {:#?}", user.friends);
    for f in &user.friends {
        assert_eq!(f.name, "Friend name");
    }
}

#[test]
fn debug_and_display_do_not_panic() {
    let user = User::new();
    let s = format!("{user:#?}");
    assert!(s.contains("User"));
}

#[test]
fn wrappers_and_dynamic_building() -> Result<(), String> {
    let id = UserId::new("00000000-0000-0000-0000-000000000000")?;
    let name = UserName::new("Alice")?;
    let password = UserPassword::new("s3cr3t")?;
    let age = UserAge::new(30)?;
    let rating = UserRating::new(0.77)?;

    let user = User::new()
        .id(id)
        .name(name)
        .password(password)
        .age(age)
        .rating(rating);

    assert_eq!(user.id, "00000000-0000-0000-0000-000000000000");
    assert_eq!(user.name, "Alice");
    assert_eq!(user.password, "s3cr3t");
    assert_eq!(user.age, 30);
    assert_eq!(user.rating, 0.77);
    assert_eq!(user.email, "johndoe@example.com");
    Ok(())
}

#[test]
fn very_long_email_and_names() {
    let local = "a".repeat(64);
    let domain = "b".repeat(60) + ".com";
    let long = format!("{}@{}", local, domain);
    let res = UserEmail::new(&long);
    match res {
        Ok(v) => assert_eq!(v.0, long),
        Err(e) => assert_eq!(e, "My custom error"),
    }
}

#[test]
fn bulk_create_valid_users() -> Result<(), String> {
    for i in 18..=30 {
        let u = User::new()
            .id(UserId::new(format!("id-{i}"))?)
            .name(UserName::new(format!("Name{i}"))?)
            .password(UserPassword::new("pwd")?)
            .age(UserAge::new(i as u8)?);
        assert!(u.age >= 18 && u.age <= 125);
    }
    Ok(())
}

#[test]
fn set_email_with_wrapper() -> Result<(), String> {
    let email = UserEmail::new("foo.bar@example.com")?;
    let user = User::new().email(email);
    assert_eq!(user.email, "foo.bar@example.com");
    Ok(())
}

#[test]
fn friend_default_name() {
    let f = Friend::new();
    assert_eq!(f.name, "Friend name");
}

#[test]
fn best_friend_default_name() {
    let user = User::new();
    assert_eq!(user.best_frined.name, "Friend name");
}

#[test]
fn builder_chain_accepts_wrappers() -> Result<(), String> {
    let u = User::new()
        .id(UserId::new("id-1")?)
        .name(UserName::new("B")?)
        .password(UserPassword::new("p")?)
        .email(UserEmail::new("b@b.com")?)
        .age(UserAge::new(20)?)
        .rating(UserRating::new(0.3)?);
    assert_eq!(u.email, "b@b.com");
    assert_eq!(u.id, "id-1");
    assert_eq!(u.name, "B");
    Ok(())
}

#[test]
fn replace_friends_vector() -> Result<(), String> {
    let friend = Friend::new().name(FriendName::new("New Friend")?);
    let mut user = User::new();
    user = user.friends(UserFriends::new(vec![friend])?);
    assert_eq!(user.friends.len(), 1);
    assert_eq!(user.friends[0].name, "New Friend");
    Ok(())
}
