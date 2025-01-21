# Compiler le code Rust vers WASM :

### Dans le dossier racine (où se trouve le Cargo.toml)
```wasm-pack build```

Cela va :

- Compiler le code Rust présent dans src/
- Générer les bindings WASM
- Créer un dossier pkg avec le WASM compilé