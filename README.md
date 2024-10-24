# Mon projet Rust

Voici un exemple de code qui ne doit pas compiler :

```rust,compile_fail
fn example_fail() {
    let mut value = 10;
    let ref_value = &value;
    value = 20;  // Erreur : mutation après emprunt immuable
}

