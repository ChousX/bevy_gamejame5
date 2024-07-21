use crate::prelude::*;

#[derive(Debug, Component)]
enum Gender {
    Male,
    Female,
    NonBinary,
    Other(String),
}
