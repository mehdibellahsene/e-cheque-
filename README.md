# e-cheque

Module de communication POS (caisse) pour la lecture des données de chèque, écrit en Rust.

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (toolchain stable, ≥ 1.85)

## Commandes

| Action              | Commande                              |
| ------------------- | ------------------------------------- |
| Compiler            | `cargo build`                         |
| Lancer              | `cargo run`                           |
| Tests unitaires     | `cargo test`                          |
| Formatage           | `cargo fmt --all --check`             |
| Lint                | `cargo clippy --all-targets`          |

## Intégration continue

Le workflow GitHub Actions (`.github/workflows/ci.yml`) vérifie à chaque push et pull request :
formatage, lint (clippy, warnings bloquants), tests et exécution.
