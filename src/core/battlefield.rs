use cgmath::Rad;
use core::position::Position;
use core::unit::{Indiv, IndivId, UnitTypeId};
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use types::Size2;

#[derive(Clone, Debug)]
pub struct Battlefield {
    indivs: HashMap<IndivId, Indiv>,
    //companies: HashMap<CompId, Vec<indivId>>,
    pub map_size: Size2,
    next_indiv_id: u32,
}

impl Battlefield {
    pub fn new() -> Battlefield {
        let indivs: HashMap<IndivId, Indiv> = HashMap::new();

        let mut battlefield = Battlefield {
            indivs,
            map_size: Size2 { w: 5, h: 5 },
            next_indiv_id : 0,
        };
        for i in 0..5 {
            for j in 0..5 {
            battlefield.add_indiv_at(Position::new(i as f64, j as f64));
            }
        }
        battlefield
    }

    fn add_indiv_at(&mut self, pos: Position){
        let id = self.next_indiv_id;
        self.add_indiv(&Indiv{
            id: IndivId { id  },
            pos,
            rot: Rad(0.0),
            player_id: 0, // Replace by PlayerID?
            type_id: UnitTypeId { id: 0 },
            hp: 0,
            xp: 0,
        });
        self.next_indiv_id += 1;
    }

    fn add_indiv(&mut self, indiv: &Indiv) {
        assert!(self.indivs.get(&indiv.id).is_none());
        self.indivs.insert(indiv.id, indiv.clone());
    }

    pub fn get_indiv(&self, indiv_id: &IndivId) -> Option<&Indiv> {
        self.indivs.get(indiv_id)
    }

    pub fn get_indiv_iter(&self) -> Iter<IndivId, Indiv> {
        self.indivs.iter()
    }
}
