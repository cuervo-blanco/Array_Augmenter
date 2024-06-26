use std::cell::RefCell;
use std::thread;
use std::rc::Rc;
use core::time::Duration;

use crate::character_creation_menu::utils;
use crate::character_creation_menu::leader;
use crate::character_creation_menu::leader::Leader;
use crate::character_creation_menu::skills::Skills;
use crate::character_creation_menu::position::Position;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Hat {
    name: String,
}
#[derive(Debug)]
#[derive(Clone)]
pub struct Astronauts {
    senior: Leader,
}
impl Astronauts {
    pub fn push(&mut self, name: String, attributes: Vec<(String, i8)>, skill: Skills, position: Position) {
        let astronaut: Rc<RefCell<Astronaut>> = Rc::new(RefCell::new(Astronaut {
            name: name.trim().to_string(),
            life: 10.0,
            skill,
            position,
            // B.R.I.G.H.T.S.
            brainpower: attributes[0].1,
            reflex: attributes[1].1,
            influence: attributes[2].1,
            good_fortune: attributes[3].1,
            heart: attributes[4].1,
            toughness: attributes[5].1,
            sight: attributes[6].1,
            // Pointer
            senior: self.senior.clone_astro(),
        }));
        self.senior = leader::Leader::Astronaut(astronaut.clone());
    }
    #[allow(dead_code)]
    fn pop(&mut self) -> Option<String> {
        let senior = match &self.senior {
            Leader::Astronaut(astronaut) => astronaut.borrow().senior.clone_astro(),
            _ => return None,
        };
        self.senior = senior;
        match &self.senior {
            Leader::Astronaut(astronaut) => Some(astronaut.borrow().name.clone()),
            _ => None,
        }
    }
    fn print_recursive(leader: &Leader) {
        println!("");
        match leader {
            Leader::Astronaut(astronaut_rc) => {
                let astronaut = astronaut_rc.borrow();
                Astronauts::print_recursive(&astronaut.senior);
                println!("{}: {} {}", astronaut.name, astronaut.skill, astronaut.position);
                println!("B:{}, R:{}, I:{}, G:{}, H:{}, T:{}, S:{}",
                    astronaut.brainpower,
                    astronaut.reflex,
                    astronaut.influence,
                    astronaut.good_fortune,
                    astronaut.heart,
                    astronaut.toughness,
                    astronaut.sight
                );
                println!("");
            }
            Leader::Hat(har_rc) => {
                let _hat = har_rc.borrow();
                println!("");
            }
            Leader::None => {
                println!("No leader found");
            }
        }
    }
    pub fn print_name(&self) {
        Astronauts::print_recursive(&self.senior);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Astronaut {
    // Characteristics
    name: String,
    life: f32,
    skill: Skills,
    position: Position,
    // B.R.I.G.H.T.S Player Attributes
    brainpower: i8,
    reflex: i8,
    influence: i8,
    good_fortune: i8,
    heart: i8,
    toughness: i8,
    sight: i8,
// Pointer
    senior: Leader
}
#[allow(dead_code)]
impl Astronaut {
    fn heal(&mut self, amount: f32) {
        self.life += amount;
    }
    fn hurt(&mut self, amount: f32) {
        self.life -= amount;
    }
}

pub fn create_astronaut_list() -> Astronauts {
    // Ask more information about what to add and then kick
    println!("Starting creation process...");
    thread::sleep(Duration::from_millis(1000));
    println!("");

    // Captain hat creation
    let captain_hat: Rc<RefCell<Hat>> = Rc::new(RefCell::new(Hat {
        name: "Captain's Hat!".to_string(),
    }));
    //Astronauts list initiation process
    let astronaut_list: Astronauts = Astronauts {
        senior: Leader::Hat(captain_hat),
    };

    astronaut_list
}



pub fn astronaut_attributes() -> Vec<(String, i8)> {

    let mut attributes: Vec<(String, i8)> = vec![
        ("Brain Power".to_string(), 0),
        ("Reflex".to_string(), 0),
        ("Influence".to_string(), 0),
        ("Good Fortune".to_string(), 0),
        ("Heartfulness".to_string(), 0),
        ("Toughness".to_string(), 0),
        ("Sight".to_string(), 0)];

    let mut total_points: i8 = 30;

    let mut input = String::new();

    println!("Astronaut creation process initializing...");
    thread::sleep(Duration::from_millis(1000));
    println!("B.R.I.G.H.T.S. successfuly intialized");
    thread::sleep(Duration::from_millis(1000));
    loop {
        println!("Type HELP for help.");
        println!("");
        println!("Total points left: {}", total_points);
        for (i, (name, value)) in attributes.iter().enumerate() {
            println!("{}. {}: {}", i+1, name, value);
        }
        println!("");

        println!("Enter the parameter number to change (0 to finish): ");
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.trim().to_uppercase() == "HELP" {
            utils::print_help(1);
        } else {
            let choice: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if choice == 0 {
                break;
            } else if choice > 0 && choice <= 7 {
                let attribute_index = (choice - 1) as usize;
                println!("Enter new value for {}: ", attributes[attribute_index].0);
                input.clear();
                std::io::stdin().read_line(&mut input).expect("Failed ot read input");
                let new_value: i8 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if new_value > 10 {
                    println!("Maximum 10 points per attribute.");
                    thread::sleep(Duration::from_millis(2000));
                } else if new_value <= total_points {
                    total_points -= new_value - attributes[attribute_index].1;
                    attributes[attribute_index].1 = new_value;
                } else {
                    println!("Not enough points.");
                }
            }
        }
    }
    attributes
}


