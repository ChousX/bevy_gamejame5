use crate::prelude::*;
// import the macros needed
// You need to import the trait, to have access to VARIANTS
use strum::{VariantNames, EnumString};

#[derive(Debug, Component, EnumString, VariantNames)]
#[strum(serialize_all = "kebab-case")]
enum LivingThing {
    Animal(Animal),
    Plant(Plant),
    Microorganism(Microorganism),
}
impl Default for LivingThing{ fn default() -> Self { Self::Animal(Animal::default())} }


#[derive(Debug, EnumString, VariantNames)]
#[strum(serialize_all = "kebab-case")]
enum Animal {
    Mammal(Mammal),
    Bird(Bird),
    Fish(Fish),
    Reptile(Reptile),
    Amphibian(Amphibian),
    Insect(Insect),
    Other(String),
}

impl Default for Animal{ fn default() -> Self { Self::Mammal(Mammal::default())} }

#[derive(Debug, EnumString, VariantNames, Default)]
#[strum(serialize_all = "kebab-case")]
enum Mammal {
    #[default]
    Human,
    Dog,
    Cat,
    Elephant,
    Whale,
    Dolphin,
    Lion,
    Tiger,
    Bear,
    Monkey,
    Sloth,
    Horse,
    Cow,
    Sheep,
    Goat,
    Bat,
    Seal,
    PolarBear,
    Panda,
    Kangaroo,
    Gorilla,
    Chimpanzee,
    Otter,
    Hedgehog,
    Squirrel,
    Other(String),
}


#[derive(Debug, EnumString, VariantNames, Default)]
#[strum(serialize_all = "kebab-case")]
enum Bird {
    Eagle,
    Owl,
    Penguin,
    Ostrich,
    #[default]
    Parrot,
    Other(String),
}

#[derive(Debug, EnumString, VariantNames, Default)]
#[strum(serialize_all = "kebab-case")]
enum Fish {
    Shark,
    Salmon,
    #[default]
    Trout,
    Clownfish,
    Other(String),
}

#[derive(Debug, EnumString, VariantNames, Default)]
#[strum(serialize_all = "kebab-case")]
enum Reptile {
    #[default]
    Turtle,
    Crocodile,
    Lizard,
    Snake,
    Other(String),
}

#[derive(Debug, EnumString, VariantNames, Default)]
#[strum(serialize_all = "kebab-case")]
enum Amphibian {
    #[default]
    Frog,
    Toad,
    Salamander,
    Other(String),
}

#[derive(Debug, EnumString, VariantNames, Default)]
#[strum(serialize_all = "kebab-case")]
enum Insect {
    Butterfly,
    Bee,
    Ant,
    #[default]
    Ladybug,
    Dragonfly,
    Other(String),
}

#[derive(Debug, EnumString, VariantNames, Default)]
#[strum(serialize_all = "kebab-case")]
enum Plant {
    Tree,
    Flower,
    #[default]
    Grass,
    Moss,
    Fern,
    Other(String),
}

#[derive(Debug, EnumString, VariantNames, Default)]
#[strum(serialize_all = "kebab-case")]
enum Microorganism {
    #[default]
    Bacteria,
    Virus,
    Fungus,
    Algae,
    Protozoa,
    Other(String),
}
