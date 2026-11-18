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

## Écrire les tests

Le projet suit le cycle TDD : **Red** (écrire un test qui échoue), **Green** (changement minimal
pour que tous les tests passent), **Refactor** (remanier sans casser les tests). Un bug signalé
commence toujours par un test qui le reproduit.

- **Organisation** : les tests vivent dans `tests/`, un fichier par module testé
  (`tests/<module>.rs`). Ils n'utilisent que l'API publique et testent le comportement,
  pas l'implémentation.
- **Un cas = une fonction de test**, nommée `<action>_<condition>_<résultat attendu>`.
  Chaque comportement est couvert par un cas nominal, des cas extrêmes et des cas d'erreur.
- **Structure AAA** : chaque test est découpé en `// Arrange`, `// Act`, `// Assert`.
- **Autonomes et déterministes** : aucun test n'accède au matériel, au réseau, au système de
  fichiers ou à l'horloge. Ces dépendances sont injectées via un trait et remplacées par un
  *stub* (réponses prédéfinies) ou un *mock* (vérification des interactions).
- **Rapides** : la suite complète s'exécute en quelques secondes.

## Règles de code

- Tout comportement nouveau ou corrigé est couvert par un test.
- Les dépendances externes (port série, horloge…) sont injectées, jamais instanciées en dur.
- Les éléments publics sont documentés, avec un exemple lorsque c'est pertinent.
- Le code `unsafe` est interdit.
- Les erreurs métier sont exprimées par des types dédiés, jamais par `panic!` ou `unwrap()`
  dans le code de production.

## Licence

En contribuant, vous acceptez que votre contribution soit distribuée sous la double licence
MIT OR Apache-2.0 du projet.
