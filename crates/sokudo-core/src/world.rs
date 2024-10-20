use sokudo_io::{read::ParsedWorld, write::{collider::WriteCollider, WriteWorldState}};

use crate::collider::Collider;

pub struct World {
    pub steps: u32,
    pub colliders: Vec<Collider>,
}

impl World {
    pub fn step(&mut self) {
        for collider in self.colliders.iter_mut() {
            collider.transform.translate.y -= 0.1;
        }
    }

    pub fn state(&self) -> WriteWorldState {
        WriteWorldState {
            colliders: self.colliders.iter().map(WriteCollider::from).collect(),
        }
    }
}

impl From<ParsedWorld> for World {
    fn from(value: ParsedWorld) -> Self {
        World {
            steps: value.steps,
            colliders: value.colliders.into_iter().map(Collider::from).collect(),
        }
    }
}
