#[derive(Debug)]
enum Couleur {
    Coeur,
    Trefle,
    Carreau,
    Pique,
}

#[derive(Debug)]
enum Valeur {
    Numero(u8), // Valeur entre 1 et 10
    Valet,
    Dame,
    Roi,
}

#[derive(Debug)]
struct Carte {
    couleur: Couleur,
    valeur: Valeur,
}

fn main() {
    let mut cartes: Vec<Carte> = Vec::new();

    // Ajouter les cartes numériques (1 à 10)
    for couleur in &[Couleur::Coeur, Couleur::Trefle, Couleur::Carreau, Couleur::Pique] {
        for valeur in 1..=10 {
            cartes.push(Carte {
                couleur: couleur.clone(),
                valeur: Valeur::Numero(valeur),
            });
        }
    }

    // Ajouter les figures (Valet, Dame, Roi)
    for couleur in &[Couleur::Coeur, Couleur::Trefle, Couleur::Carreau, Couleur::Pique] {
        cartes.push(Carte {
            couleur: couleur.clone(),
            valeur: Valeur::Valet,
        });
        cartes.push(Carte {
            couleur: couleur.clone(),
            valeur: Valeur::Dame,
        });
        cartes.push(Carte {
            couleur: couleur.clone(),
            valeur: Valeur::Roi,
        });
    }

    // Afficher toutes les cartes
    for carte in cartes {
        dbg!(&carte);
    }
}
