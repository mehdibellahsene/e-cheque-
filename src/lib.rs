//! Bibliothèque e-cheque : module de communication avec la caisse (POS).

/// Message d'accueil de l'application.
pub fn greeting() -> &'static str {
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
