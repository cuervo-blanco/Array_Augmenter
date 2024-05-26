// that can also work like a stack with a pop and push capabilities
// which we will call differently.
// we must also perform search of that array to find a particular name

use std::cell::RefCell;
use std::fmt;
use std::thread;
use std::ptr::addr_of;
use std::io;
use std::rc::Rc;

use core::time::Duration;
// External Crates
use once_cell::sync::Lazy;

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
    fn push(&mut self, name: String, attributes: Vec<(String, i8)>, skill: Skills, position: Position) {
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Position {
    Captain,
    FirstOfficer,
    Communications,
    Doctor,
    Engineer
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Position::Captain => write!(f, "Captain"),
            Position::FirstOfficer => write!(f, "First Officer"),
            Position::Communications => write!(f, "Communications Officer"),
            Position::Doctor => write!(f, "Doctor"),
            Position::Engineer => write!(f, "Engineer")
        }
    }
}


#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Skills {
    ComputerNerd,
    Medic,
    Negotiator,
    Doctor,
    Scientist,
    Hacker,
    Survivalist,
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

impl fmt::Display for Skills {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Skills::ComputerNerd => write!(f, "Computer Nerd"),
            Skills::Medic => write!(f, "Medic"),
            Skills::Negotiator => write!(f, "Negotiator"),
            Skills::Doctor => write!(f, "Doctor"),
            Skills::Scientist => write!(f, "Scientist"),
            Skills::Hacker => write!(f, "Hacker"),
            Skills::Survivalist => write!(f, "Survivalist"),
            Skills::Leader => write!(f, "Leader"),
            Skills::Strategist => write!(f, "Strategist"),
            Skills::Mechanic => write!(f, "Mechanic"),
            Skills::Biologist => write!(f, "Biologist"),
            Skills::Navigator => write!(f, "Navigator"),
            Skills::CommunicationsSpecialist => write!(f, "Communications Specialist"),
            Skills::Technician => write!(f, "Technician"),
            Skills::SecurityExpert => write!(f, "Security Expert"),
            Skills::Chemist => write!(f, "Chemist"),
            Skills::Astrophysicist => write!(f, "Astrophysicist"),
            Skills::Psychologist => write!(f, "Psychologist"),
            Skills::Trader => write!(f, "Trader"),
            Skills::Engineer => write!(f, "Engineer"),
            Skills::Soldier => write!(f, "Soldier"),
            Skills::None => write!(f, "None")
        }
    }
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
                let option_title: String = String::from("2. Show Current Astronauts");
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

                println!("Enter Austronaut name: ");
                let mut name: String = String::new();
                let _ = io::stdin().read_line(&mut name);
                let attributes = character_attributes();
                let skills = character_skills();
                let position = character_position();
                astronaut_list.push(name, attributes, skills, position);
                println!("Crew Members so far: ");
                astronaut_list.print_name();
                Some(Box::new(GameObjects::AstronautList(astronaut_list)))
            }
            OptionSelector::Option2 => {
                // Should output the first structure in an array (which is the option)
                // The option then
                let astronaut_list: Astronauts;

                #[allow(irrefutable_let_patterns)]
                if let Some(ref boxed_game_objects) = self.list {
                    if let GameObjects::AstronautList(ref existing_list) = **boxed_game_objects {
                        astronaut_list = existing_list.clone();
                        astronaut_list.print_name();
                    } else {
                        println!("");
                        println!("There are no astronauts");
                        println!("");
                    }
                } else {
                    println!("There are no astronauts");
                }
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

#[allow(dead_code)]
fn print_help(menu: u8) {
    let help_text: &str;
    match menu {
        1  => {
        help_text = "
            *** Welcome to the Astronaut Creation Station! ***
            Ready to power up your character? You've got 30 points to distribute across 7 skills:

            - **Brainpower (Intellect)**: Boost your hacking and puzzle-solving abilities!
            - **Reflex (Agility)**: Crank up your speed and dodging skills!
            - **Influence (Charisma)**: Amp up your charisma to persuade and charm!
            - **Good Fortune (Luck)**: Increase your luck for better outcomes and rare finds!
            - **Heartfulness (Strength)**: Pump up your physical strength for heavy lifting and endurance!
            - **Toughness (Endurance)**: Enhance your resistance to damage and fatigue!
            - **Sight (Perception)**: Sharpen your awareness to spot hidden treasures and dangers!

            Remember, you've got a total of 30 points. Spend them wisely and get ready to level up!
            ";
        } 2 => {
        help_text =
            "
            ComputerNerd: \"Knows every cheat code in the book.\"
                    Boosts Brainpower (+2), reduces Heartfulness (-2).

            Medic: \"Can patch you up with duct tape and a band-aid.\"
                    Enhances Toughness (+2), lowers Reflex (-2).

            Negotiator: \"Talks their way out of any parking ticket.\"
                    Improves Influence (+2), weakens Brainpower (-2).

            Doctor: \"Flies like a leaf on the wind.\"
                    Increases Reflex (+2), diminishes Good Fortune (-2).

            Scientist: \"Likes to blow things up for science.\"
                    Elevates Brainpower (+2), reduces Toughness (-2).

            Hacker: \"One password away from ruling the world.\"
                    Raises Brainpower (+2), decreases Influence (-2).

            Survivalist: \"Could survive on Mars with a toothpick.\"
                    Boosts Toughness (+2), cuts Brainpower (-2).

            Leader: \"Rallies the troops with a single speech.\"
                    Enhances Influence (+2), lessens Reflex (-2).

            Strategist: \"Always three steps ahead.\"
                    Improves Brainpower (+2), reduces Heartfulness (-2).

            Mechanic: \"Fixes anything with a wrench and a prayer.\"
                    Elevates Toughness (+2), diminishes Sight (-2).

            Biologist: \"Knows which plants won't eat you.\"
                    Boosts Brainpower (+2), cuts Reflex (-2).

            Navigator: \"Never asks for directions.\"
                    Increases Reflex (+2), lowers Sight (-2).

            CommunicationsSpecialist: \"Talks their way through static.\"
                    Enhances Influence (+2), weakens Toughness (-2).

            Technician: \"Gizmo guru extraordinaire.\"
                    Raises Brainpower (+2), reduces Good Fortune (-2).

            SecurityExpert: \"Locks down tighter than Fort Knox.\"
                    Improves Reflex (+2), diminishes Heartfulness (-2).

            Chemist: \"Mixes potions and sometimes explosions.\"
                    Boosts Brainpower (+2), cuts Reflex (-2).

            Astrophysicist: \"Knows what stars are made of.\"
                    Enhances Brainpower (+2), lowers Influence (-2).

            Psychologist: \"Reads minds like a comic book.\"
                    Elevates Influence (+2), reduces Brainpower (-2).

            Trader: \"Always gets the best deal.\"
                    Improves Good Fortune (+2), lessens Heartfulness (-2).

            Engineer: \"Builds wonders from scrap.\"
                    Boosts Toughness (+2), diminishes Reflex (-2).

            Soldier: \"Tough as nails, strong as steel.\"
                    Increases Heartfulness (+2), weakens Good Fortune (-2).

            None: \"Just your average space cadet.\"
                    No effect on attributes.";
        } 3 => help_text = "
            Captain:
                    Responsible for piloting the ship.
                    Any issues with the captain affect the ship's navigation and overall command.
            First Offier:
                    Handles crisis management and suppors the Captain.
                    Problems with the First Officer lead to slower crisis resolution and lower crew efficiency.
            Communications Officer:
                    Manages external communications and detects incoming threats.
                    Issues with this officer result in missed warnings and poor diplomatic interactions.
            Doctor:
                    Ensures crew health and treats injuries.
                    If the Doctor is incapacitated, crew recovery and health maintenance are severly hindered.
            Engineer:
                    Maintains and repairs the ship's systems.
                    Problems with the Engineer result in longer repair times and potential loss of ship abilities.
            ",
        _ => {
            println!("Not a valid option");
            help_text = "";
        }
    }

    println!("{}", help_text);
}

fn character_skills() -> Skills {
    loop {
        let skills: Vec<Skills> = vec![
            Skills::ComputerNerd,
            Skills::Medic,
            Skills::Negotiator,
            Skills::Doctor,
            Skills::Scientist,
            Skills::Hacker,
            Skills::Survivalist,
            Skills::Leader,
            Skills::Strategist,
            Skills::Mechanic,
            Skills::Biologist,
            Skills::Navigator,
            Skills::CommunicationsSpecialist,
            Skills::Technician,
            Skills::SecurityExpert,
            Skills::Chemist,
            Skills::Astrophysicist,
            Skills::Psychologist,
            Skills::Trader,
            Skills::Engineer,
            Skills::Soldier,
            Skills::None
        ];
        let mut input = String::new();
        println!("Select Character Skill");
        println!("Type HELP for more information");
        println!("");

        for (i, skill ) in skills.iter().enumerate() {
            println!("{}. {}", i+1, skill);
            thread::sleep(Duration::from_millis(100));

        }

        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.trim().to_uppercase() == "HELP" {
            print_help(2);
        } else {
            let choice: u8 = match input.trim().parse::<u8>() {
                Ok(num) => num - 1,
                Err(_) => continue,
            };
            if choice <= 22 {
                let skill = skills[choice as usize];
                return skill
            } else {
                println!("Invalid option");
            }
        }

    }


}

static mut POSITIONS: Lazy<Vec<Position>> = Lazy::new(|| vec![Position::Captain,
    Position::FirstOfficer,
    Position::Communications,
    Position::Doctor,
    Position::Engineer
]);

fn character_position() -> Position {

    let mut input = String::new();

    println!("Which Position would you like your character to have?");
    println!("");
    loop {
        unsafe {
            for (i, skill ) in POSITIONS.iter().enumerate() {
                println!("{}. {}", i+1, skill);
                thread::sleep(Duration::from_millis(100));
            }
        }
        println!("");
        println!("Write HELP for more information.");

        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.trim().to_uppercase() == "HELP" {
            print_help(3);
        } else {
            let choice: u8 = match input.trim().parse::<u8>() {
                Ok(num) => num - 1,
                Err(_) => continue,
            };
            if choice <= 4 {
                unsafe {
                    let position = POSITIONS[choice as usize];
                    POSITIONS.remove(choice.into());
                    return position
                }
            } else {
                println!("Invalid option");
            }
        }
    }
}

fn character_attributes() -> Vec<(String, i8)> {

    let mut skills: Vec<(String, i8)> = vec![
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
        for (i, (name, value)) in skills.iter().enumerate() {
            println!("{}. {}: {}", i+1, name, value);
        }
        println!("");

        println!("Enter the parameter number to change (0 to finish): ");
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.trim().to_uppercase() == "HELP" {
            print_help(1);
        } else {
            let choice: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if choice == 0 {
                break;
            } else if choice > 0 && choice <= 7 {
                let skill_index = (choice - 1) as usize;
                println!("Enter new value for {}: ", skills[skill_index].0);
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
                1..=3 => {
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
