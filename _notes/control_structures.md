# Control Structures & Errors

- Avoid deep nesting
- Use Factory Functions & Polymorphism
- Prefer Positive Conditions
- Handle Errors

## Using Guard Clauses

Guard clauses are early returns that help to avoid deep nesting.

```rust
fn guard_clauses(user: User) {
    if user.is_admin() {
        return;
    }

    if user.is_active() {
        return;
    }

    if user.is_verified() {
        return;
    }

    // Do something
}
```
