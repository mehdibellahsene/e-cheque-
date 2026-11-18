# Guide de contribution

Merci de votre intérêt pour e-cheque. Ce guide décrit comment proposer une modification.

## Signaler un problème

Avant d'ouvrir une issue, vérifiez qu'elle n'existe pas déjà. Utilisez le modèle adapté
(bug, demande de fonctionnalité, mutant survivant) et fournissez un exemple reproductible.

## Proposer une modification

1. Créez une branche à partir de `main` : `git switch -c feat/ma-fonctionnalite`.
2. Faites des commits atomiques respectant [Conventional Commits](https://www.conventionalcommits.org/fr/) :

   | Type       | Usage                                         |
   | ---------- | --------------------------------------------- |
   | `feat`     | Nouvelle fonctionnalité                       |
   | `fix`      | Correction de bug                             |
   | `test`     | Ajout ou modification de tests                |
   | `refactor` | Restructuration sans changement de comportement |
   | `docs`     | Documentation                                 |
   | `ci`       | Intégration continue                          |
   | `chore`    | Maintenance (dépendances, configuration)      |

   Référencez les issues concernées dans le message (`Closes #12`).
3. Vérifiez localement que tout passe avant de pousser :

   ```sh
   cargo fmt --all --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   ```

4. Ouvrez une pull request vers `main` en remplissant le modèle fourni.

## Règles de code

- Tout comportement nouveau ou corrigé est couvert par un test.
- Les éléments publics sont documentés, avec un exemple lorsque c'est pertinent.
- Le code `unsafe` est interdit.
- Les erreurs métier sont exprimées par des types dédiés, jamais par `panic!` ou `unwrap()`
  dans le code de production.

## Licence

En contribuant, vous acceptez que votre contribution soit distribuée sous la double licence
MIT OR Apache-2.0 du projet.
