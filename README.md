### Capsule Global State

This is an MRE showing an alleged issue with using global state at build time in capsules.
Essentially, the capsule needs to be delayed for the build to not panic.
From what I have seen, we run into the unreachable case in the global state logic.

Removing the use of global state from the index template lets the developer use the non-delayed capsule.

### To run

`$ API_URL="Some arbitrary value" perseus serve`

### To reproduce

in `src/templates/index.rs` comment/uncomment those 2 lines:

```rust
(CAPSULE.delayed_widget(cx,"",())) // This works properly but is unecessarily delayed
// (CAPSULE.widget(cx,"",())) // This makes the build panic when using global state
```
