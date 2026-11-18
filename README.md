# e-cheque

[![CI](https://github.com/mehdibellahsene/e-cheque-/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/mehdibellahsene/e-cheque-/actions/workflows/ci.yml)
[![Licence : MIT OR Apache-2.0](https://img.shields.io/badge/licence-MIT%20OR%20Apache--2.0-blue.svg)](#licence)
[![MSRV](https://img.shields.io/badge/MSRV-1.85-orange.svg)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)

Module de communication avec la caisse (POS) permettant à un périphérique de lecture de chèques
de récupérer la date et le montant de la transaction en cours.

## Protocole

Le périphérique dialogue avec la caisse via une liaison série (port COM / flux d'octets).

| Sens                  | Trame                    | Signification                          |
| --------------------- | ------------------------ | -------------------------------------- |
| Périphérique → caisse | `GET_CHEQUE_DATA\n`      | Demande des données du chèque          |
| Caisse → périphérique | `OK;YYYY-MM-DD;MONTANT\n` | Réponse nominale (ex. `OK;2026-11-16;145.50\n`) |
| Caisse → périphérique | `ERR_NO_TRANSACTION\n`   | Aucun paiement par chèque en cours     |
| Caisse → périphérique | `ERR_BUSY\n`             | Caisse occupée                         |

Toute trame malformée, vide ou absente (timeout) est traitée comme une erreur.

## Démarrage rapide

Prérequis : [Rust](https://www.rust-lang.org/tools/install) 1.85 ou plus récent.

```sh
git clone https://github.com/mehdibellahsene/e-cheque-.git
cd e-cheque-
cargo run
```

## Développement

| Action          | Commande                                       |
| --------------- | ---------------------------------------------- |
| Compiler        | `cargo build`                                  |
| Tester          | `cargo test`                                   |
| Formater        | `cargo fmt --all`                              |
| Analyser        | `cargo clippy --all-targets -- -D warnings`    |
| Documentation   | `cargo doc --no-deps --open`                   |

L'intégration continue ([`ci.yml`](.github/workflows/ci.yml)) exécute ces vérifications sur
Linux, Windows et macOS à chaque pull request et à chaque push sur `main`.

## Structure

```text
.
├── src/
│   ├── lib.rs        # Bibliothèque : logique de communication POS
│   └── main.rs       # Point d'entrée de l'application
├── .github/          # CI, Dependabot, modèles d'issue et de pull request
└── Cargo.toml
```

## Contribuer

Les contributions sont les bienvenues. Lisez le [guide de contribution](CONTRIBUTING.md) et le
[code de conduite](CODE_OF_CONDUCT.md) avant d'ouvrir une issue ou une pull request.
Pour signaler une vulnérabilité, suivez la [politique de sécurité](SECURITY.md).

## Licence

Ce projet est distribué sous la licence de votre choix parmi :

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

Sauf mention contraire explicite, toute contribution soumise pour inclusion dans ce projet,
telle que définie par la licence Apache-2.0, est distribuée sous cette double licence, sans
termes ni conditions supplémentaires.
