use std::io;

struct livre{
    titre: String,
    description: String,
    nbr_page: i32,
}

impl livre{
    fn creer_livre(titre: String, nbr_page: i32) -> livre {
        livre{
            titre,
            description: String::from(""),
            nbr_page,
        }
    }
    
    fn nouvelle_description(self, description: String) -> livre {
        livre{
            description,
            ..self
        }
    }
    
    fn changer_description(&mut self, d: String){
        self.description = d;
    }
}

fn dupliquer_livre(l: &livre) -> livre {
    livre{
        titre: l.titre.clone(),
        description: l.description.clone(),
        nbr_page: l.nbr_page.clone(),
    }
}

fn main() {
    let mut l1 = livre::creer_livre(String::from("Rust Book"), 675);
    let mut description_utilisateur = String::new();
    println!("Tapez une description :");
    io::stdin()
        .read_line(&mut description_utilisateur)
        .expect("Erreur lors de la lecture de la description.");
    let description_utilisateur = description_utilisateur.trim().to_string();
    l1.changer_description(description_utilisateur);
    dbg!(l1.titre, l1.description, l1.nbr_page);
}