//! Communication avec la caisse (POS) pour la lecture des données de chèque.

/// Retourne le message d'accueil de l'application.
///
/// # Exemple
///
/// ```
/// assert_eq!(e_cheque::greeting(), "Hello, world!");
/// ```
#[must_use]
pub const fn greeting() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_returns_hello_world() {
        assert_eq!(greeting(), "Hello, world!");
    }
}
