# Refute

Allows to use `refute!(smth)` macro instead of `assert!(!smth)`

## Example

```rust
#[test]
fn it_works() {
    refute!(false);
}

#[test]
fn default_user_is_not_an_admin() {
    refute!(User::default().admin)
}
```
