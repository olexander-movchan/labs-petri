use std::rc::Rc;
use std::cell::RefCell;
use rand::{thread_rng, prelude::*};

pub type PlaceRef = Rc<RefCell<Place>>;

#[derive(Debug)]
pub struct PTNet {
    places: Vec<PlaceRef>,
    transitions: Vec<Transition>,
}

impl PTNet {
    pub fn new() -> Self {
        PTNet {
            places: vec![],
            transitions: vec![]
        }
    }

    pub fn place(&mut self, tag: &str, tokens: usize) -> PlaceRef {
        log::info!("Created place with tag {:?}", tag);
        let place = Place::new(tag, tokens);
        self.places.push(place.clone());
        place
    }

    pub fn transition(&mut self, tag: &str) -> &mut Transition {
        log::info!("Created transition with tag {:?}", tag);
        let transition = Transition::new(tag);
        self.transitions.push(transition);
        self.transitions.last_mut().unwrap()
    }

    pub fn fire_enabled(&mut self) -> bool {
        let mut shuffled = {
            let mut shuffled = self.transitions.clone();
            shuffled.shuffle(&mut thread_rng());
            shuffled
        };

        let mut fired = Vec::new();
        for tr in shuffled.iter_mut() {
            if tr.is_enabled() {
                log::info!("Fired transition {:?}", tr.tag);
                tr.fire_inputs();
                fired.push(tr);
            }
        }

        for tr in fired.iter_mut() {
            tr.fire_outputs()
        }

        !fired.is_empty()
    }

    pub fn run<CB>(&mut self, max_iter: usize, mut callback: CB) -> usize
    where
        CB: FnMut() -> ()
    {
        for i in 0..max_iter {
            log::info!("Running iteration {}", i);
            if !self.fire_enabled() {
                return i;
            }
            callback();
        }

        max_iter
    }

    pub fn log_state(&self) {
        for place_ref in self.places.iter() {
            let place = place_ref.borrow();
            log::info!("{} \t{}", place.name, place.tokens);
        }
    }
}

#[derive(Debug)]
pub struct Place {
    pub name: String,
    pub tokens: usize,
}

impl Place {
    pub fn new(name: &str, tokens: usize) -> PlaceRef {
        Rc::new(RefCell::new(Self {
            name: name.to_string(),
            tokens,
        }))
    }
}

#[derive(Debug, Clone)]
struct Arc {
    pub place: PlaceRef,
    pub weight: usize,
}

#[derive(Debug, Clone)]
pub struct Transition {
    tag: String,
    inputs: Vec<Arc>,
    outputs: Vec<Arc>,
}

impl Transition {
    pub fn new(tag: &str) -> Self {
        Self { tag: tag.to_string(), inputs: vec![], outputs: vec![] }
    }

    pub fn input(&mut self, place: PlaceRef, weight: usize) -> &mut Self {
        self.inputs.push(Arc { place, weight });
        self
    }

    pub fn output(&mut self, place: PlaceRef, weight: usize) -> &mut Self {
        self.outputs.push(Arc { place, weight });
        self
    }

    pub fn is_enabled(&self) -> bool {
        self.inputs
            .iter()
            .map(|a| a.place.borrow().tokens >= a.weight)
            .fold(true, |acc, available| acc && available)
    }

    pub fn fire_inputs(&mut self) {
        for arc in self.inputs.iter_mut() {
            let mut place = arc.place.borrow_mut();
            place.tokens -= arc.weight;
        }
    }

    pub fn fire_outputs(&mut self) {
        for arc in self.outputs.iter_mut() {
            let mut place = arc.place.borrow_mut();
            place.tokens += arc.weight;
        }
    }
}
