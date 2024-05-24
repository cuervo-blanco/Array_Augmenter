// that can also work like a stack with a pop and push capabilities
// which we will call differently.
// we must also perform search of that array to find a particular name

use std::cell::RefCell;
use std::thread;
use core::time::Duration;
use std::ptr::addr_of;
use std::io;
use std::rc::Rc;

#[derive(Debug)]
#[allow(dead_code)]
struct Hat {
    name: String,
}
#[derive(Debug)]
#[derive(Clone)]
struct Astronauts {
    senior: Leader,
}
#[allow(dead_code)]
impl Astronauts {
    fn push(&mut self, name: String) {
        let astronaut: Rc<RefCell<Astronaut>> = Rc::new(RefCell::new(Astronaut {
            name: name.trim().to_string(),
            life: 10.0,
            skill: Skills::None,
            // B.R.I.G.H.T.S.
            brainpower: 0,
            reflex: 0,
            influence: 0,
            good_fortune: 0,
            heart: 0,
            toughness: 0,
            sight: 0,

            senior: self.senior.clone_astro(),
        }));
        self.senior = Leader::Astronaut(astronaut.clone());
    }
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
        match leader {
            Leader::Astronaut(astronaut_rc) => {
                let astronaut = astronaut_rc.borrow();
                println!("My name is {}", astronaut.name);
                Astronauts::print_recursive(&astronaut.senior);
            }
            Leader::Hat(har_rc) => {
                let hat = har_rc.borrow();
                println!("Reached leader hat: {}", hat.name);
            }
            Leader::None => {
                println!("No leader found");
            }
        }
    }
    fn print_name(&self) {
        Astronauts::print_recursive(&self.senior);
    }
}
#[derive(Debug)]
#[allow(dead_code)]
struct Astronaut {
    // Characteristics
    name: String,
    life: f32,
    skill: Skills,
    // B.R.I.G.H.T.S.
    brainpower: u8,
    reflex: u8,
    influence: u8,
    good_fortune: u8,
    heart: u8,
    toughness: u8,
    sight: u8,
// Pointer
    senior: Leader
}

#[derive(Debug)]
#[allow(dead_code)]
enum Skills {
    ComputerNerd,
    Medic,
    Negotiator,
    Pilot,
    Scientist,
    Hacker,
    Survavilist,
    Leader,
    Strategist,
    Mechanic,
    Biologist,
    Navigator,
    CommunicationsSpecialist,
    Technician,
    SecurityExpert,
    Chemist,
    Astrophysicist,
    Psychologist,
    Trader,
    Engineer,
    Soldier,
    None
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
enum Leader {
    Astronaut(Rc<RefCell<Astronaut>>),
    Hat(Rc<RefCell<Hat>>),
    None,
}
impl Leader {
    fn clone_astro(&self) -> Leader {
        match self {
            Leader::Astronaut(astronaut) => {
                Leader::Astronaut(astronaut.clone())
            }
            Leader::Hat(hat) => Leader::Hat(hat.clone()),
            Leader::None => Leader::None,
        }
    }
}
fn create_astronaut_list() -> Astronauts {
    // Ask more information about what to add and then kick
    // other functions based on that.
    println!("Starting creation process...");

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
#[allow(dead_code)]
enum OptionSelector {
    Option1,
    Option2,
    Option3,
}
#[allow(dead_code)]
#[derive(Debug)]
enum GameObjects {
    AstronautList(Astronauts),
}
// Terminal Options
#[allow(dead_code)]
struct TerminalOption {
    id: u32,
    func: OptionSelector,
    list: Option<Box<GameObjects>>
}
#[allow(dead_code)]
impl TerminalOption {
    fn present(&self) -> String {
        match self.func {
            OptionSelector::Option1 => {
                // Should output the first structure in an array (which is the option)
                // The option then
                let option_title: String = String::from("1. Create Astronaut");
                option_title
            }
            OptionSelector::Option2 => {
                // Should output the first structure in an array (which is the option)
                // The option then
                let option_title: String = String::from("2. Empty");
                option_title
            }
            _ => {
                let option_title: String = String::from("3. Empty");
                option_title
            }
        }
    }
    fn execute(&self) -> Option<Box<GameObjects>> {
        match self.func {
            OptionSelector::Option1 => {
                // Should output the first structure in an array (which is the option)
                // The option then
                let mut astronaut_list: Astronauts;

                #[allow(irrefutable_let_patterns)]
                if let Some(ref boxed_game_objects) = self.list {
                    if let GameObjects::AstronautList(ref existing_list) = **boxed_game_objects {
                        astronaut_list = existing_list.clone();
                    } else {
                        astronaut_list = create_astronaut_list();
                    }
                } else {
                    astronaut_list = create_astronaut_list();
                }

                println!("Enter Austronaut name; try to be nice to him: ");
                let mut name: String = String::new();
                let _ = io::stdin().read_line(&mut name);
                astronaut_list.push(name);
                astronaut_list.print_name();
                Some(Box::new(GameObjects::AstronautList(astronaut_list)))
            }
            OptionSelector::Option2 => {
                // Should output the first structure in an array (which is the option)
                // The option then
                println!("You selected option 2");
                None
            }
            _ => {
                println!("You selected another option");
                None
            }
        }
    }
}
// Terminal Options
static mut ASTRONAUT_LIST: TerminalOption = TerminalOption {
    id: 1,
    func: OptionSelector::Option1,
    list: None,
};

const OPTION_2: TerminalOption = TerminalOption {
    id: 2,
    func: OptionSelector::Option2,
    list: None,
};

const OPTION_3: TerminalOption = TerminalOption {
    id: 3,
    func: OptionSelector::Option3,
    list: None,
};

fn character_skills() -> Vec<(String, u8)> {

    let mut skills: Vec<(String, u8)> = vec![
        ("Brain Power".to_string(), 0),
        ("Reflex".to_string(), 0),
        ("Influence".to_string(), 0),
        ("Good Fortune".to_string(), 0),
        ("Heart".to_string(), 0),
        ("Toughness".to_string(), 0),
        ("Sight".to_string(), 0)];

    let mut total_points: u8 = 30;

    let mut input = String::new();

    println!("Astronaut creation process initializing...");
    thread::sleep(Duration::from_millis(3000));
    println!("B.R.I.G.H.T.S. successfuly intialized");
    thread::sleep(Duration::from_millis(1000));
    loop {
        println!("Type HELP for help.");
        println!("");
        println!("Total points left: {}", total_points);
        for (i, (name, value)) in skills.iter().enumerate() {
            println!("{}. {}: {}", i+1, name, value);
        }
        println!("");

        println!("Enter the parameter number to change (0 to finish): ");
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.trim().to_uppercase() == "HELP" {
            print_help();
        } else {
            let choice: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if choice == 0 {
                break;
            } else if choice > 0 && choice <= 7 {
                let skill_index = choice - 1;
                println!("Enter new value for {}: ", skills[skill_index][0]);
                input.clear();
                std::io::stdin().read_line(&mut input).expect("Failed ot read input");
                let new_value: u8 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if new_value <= total_points {
                    total_points -= new_value - skills[skill_index].1;
                    skills[skill_index].1 = new_value;
                } else {
                    println!("Not enough points.");
                }
            }
        }
    }

    skills
}

fn print_help() {
    let help_text = "
    *** Welcome to the Astronaut Creation Station! ***
    Ready to power up your character? You've got 30 points to distribute across 7 skills:

    - **Brainpower (Intellect)**: Boost your hacking and puzzle-solving abilities!
    - **Reflex (Agility)**: Crank up your speed and dodging skills!
    - **Influence (Charisma)**: Amp up your charisma to persuade and charm!
    - **Good Fortune (Luck)**: Increase your luck for better outcomes and rare finds!
    - **Heart (Strength)**: Pump up your physical strength for heavy lifting and endurance!
    - **Toughness (Endurance)**: Enhance your resistance to damage and fatigue!
    - **Sight (Perception)**: Sharpen your awareness to spot hidden treasures and dangers!

    Remember, you've got a total of 30 points. Spend them wisely and get ready to level up!
    ";
    println!("{}", help_text);
}

fn main() {
    // Options

    let options: Vec<&TerminalOption> = vec![unsafe { &*addr_of!(ASTRONAUT_LIST) } , &OPTION_2, &OPTION_3];
    // Astronaut linked-list

    // Selection algorithm
    loop {

        println!("Select an Option");
        println!("{}", options[0].present());
        println!("{}", options[1].present());
        println!("{}", options[2].present());

        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        let selection = selection.trim();

        if let Ok(selection_num) = selection.parse::<u32>() {

            println!("You selected {}", selection);

            match selection_num {
                1 => {
                    let n = (selection_num - 1) as usize;
                    unsafe{
                        ASTRONAUT_LIST.list = options[n].execute();
                    }
                }
                _ => {
                    println!("Not a valid option")
                }
            }
        } else {
            println!("Not a number: {:?}", selection);
        }

    }

}
