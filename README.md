# `kenzu`
[![Crates.io](https://img.shields.io/crates/v/kenzu.svg)](https://crates.io/crates/kenzu)
[![Docs.rs](https://docs.rs/kenzu/badge.svg)](https://docs.rs/kenzu)
[![License](https://img.shields.io/crates/l/kenzu.svg)](https://github.com/pas2rust/kenzu/blob/main/LICENSE)
![GitHub top language](https://img.shields.io/github/languages/top/pas2rust/kenzu?color=orange&logo=rust&style=flat&logoColor=white)
![GitHub stars](https://img.shields.io/github/stars/pas2rust/kenzu?color=success&style=flat&logo=github)
![GitHub forks](https://img.shields.io/github/forks/pas2rust/kenzu?color=orange&logo=Furry%20Network%20Network&style=flat&logoColor=white)
![Tests](https://raw.githubusercontent.com/pas2rust/badges/main/kenzu-tests.svg)
![Crates.io downloads](https://img.shields.io/crates/d/kenzu.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/pas2rust/kenzu?color=ff69b4&label=update&logo=git&style=flat&logoColor=white)


**`kenzu`** is a procedural macro crate for generating **builder patterns** in Rust — both **immutable** and **mutable**, with optional compile-time validation using field-level attributes.

Whether you're building configuration structs, domain models, or input data, `kenzu` eliminates boilerplate while increasing safety and clarity.

---

## ✨ Features

- 🧱 `Builder`: Generates **immutable**, fluent `.build()`-based builders.
- 🔁 `M_Builder`: Generates **mutable** builders using `&mut self` methods.
- 🧰 Field-level defaults with `#[set(value = "...")]`.
- 🔍 Regex-based validation with `#[build(pattern = ..., err = ...)]`.
- 🔢 Range validation with `#[range(min = ..., max = ..., err = ...)]`.
- 🧼 Skip trait generation with `#[skip_trait]`, or skip setters with `#[set(skip)]`.
- ⛔ Compile-time validation of regex patterns — invalid patterns produce macro errors.

---

## ⚙️ Installation

Add it to your `Cargo.toml`:

```bash
cargo add kenzu
```

## 🚀 Usage 

### Builder

```rust
use kenzu::Builder;

#[derive(Builder)]
pub struct User {
    pub id: String,
    
    #[set(value = "default-name")]
    pub name: String,
    
    pub password: String,
    
    #[build(pattern = r"^[^@]+@[^@]+\.[^@]+$", err = "invalid email")]
    #[set(value = "user@example.com")]
    pub email: String,
    
    #[range(min = 18, max = 99, err = "invalid age")]
    #[set(value = 18)]
    pub age: u8,
}

let user = User::new()
    .id("001")
    .name("Alice")
    .password("hunter2")
    .email("alice@example.com")
    .age(30)
    .build()
    .unwrap();
```

### Mutable Builder

```rust
use kenzu::M_Builder;

#[derive(M_Builder)]
pub struct User<'a> {
    #[set(value = "uuid")]
    pub id: String,
    
    #[set(value = "default")]
    pub name: String,
    
    pub password: String,
    
    #[build(pattern = r"^[^@]+@[^@]+\.[^@]+$", err = "invalid email")]
    #[set(value = "email@example.com")]
    pub email: String,
    
    pub fix: &'a str,
    
    #[set(value = 18)]
    pub age: u8,
    
    #[set(skip)]
    pub code: u8,
    
    pub def: String,
}

let mut user = User::new();

user.id("123")
    .name("Bob")
    .password("pass123")
    .email("bob@example.com")
    .fix("ref")
    .def("value")
    .age(42)
    .build()
    .unwrap();
```

# ❤️ Donate

[![Monero](https://img.shields.io/badge/88NKLkhZf1nTVpaSU6vwG6dwBwb9tFVSM8Lpj3YqdL1PMt8Gm7opV7aUnMYBaAC9Y6a4kfDc3fLGoMVqeSJKNphyLpLdEvC-FF6600?style=flat&logo=monero&logoColor=white)](https://github.com/pas2rust/pas2rust/blob/main/pas-monero-donate.png)
[![Bitcoin](https://img.shields.io/badge/bc1qnlayyh84e9u5pd4m9g9sf4c5zdzswvkmudmdu5-EAB300?style=flat&logo=bitcoin&logoColor=white)](https://github.com/pas2rust/pas2rust/blob/main/pas-bitcoin-donate.png)