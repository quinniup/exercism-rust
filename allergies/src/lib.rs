pub struct Allergies {
    score: u8
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score: score as u8 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & *allergen as u8 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let all_of_them = vec![
            Allergen::Eggs,
            Allergen::Pollen,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats
        ];
        all_of_them.iter().filter(|x| self.is_allergic_to(x)).cloned().collect()
    }
}