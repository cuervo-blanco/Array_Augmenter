//I want to be able to add names to an array through linked lists
// That can also work like a stack with a pop and push capabilities
// which we will call differently.
// we must also perform search of that array to find a particular name

use std::cell::RefCell;
use std::ptr::addr_of;
use std::io;
use std::rc::Rc;

struct Hat {
    name: String,
}
struct Astronauts {
    leader: AstronautPointer,
}
struct Astronaut {
    name: String,
    senior: AstronautPointer,
}
enum AstronautPointer {
    Astronaut(Rc<RefCell<Astronaut>>),
    Hat(Rc<RefCell<Hat>>),
    None,
}
impl AstronautPointer {
    fn clone_astro(&self) -> AstronautPointer {
        match self {
            AstronautPointer::Astronaut(astronaut) => {
                AstronautPointer::Astronaut(astronaut.clone())
            }
            AstronautPointer::Hat(hat) => AstronautPointer::Hat(hat.clone()),
            AstronautPointer::None => AstronautPointer::None,
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
        leader: AstronautPointer::Hat(captain_hat),
    };

    astronaut_list
}

// Create Astronaut
fn create_astronaut(list: &mut Astronauts) {
    // Get User Input for name
    println!("Enter Austronaut name; try to be nice to him: ");
    let mut name: String = String::new();
    let _ = io::stdin().read_line(&mut name);
    // Create Astronaut
    let astronaut: Rc<RefCell<Astronaut>> = Rc::new(RefCell::new(Astronaut {
        name: name.trim().to_string(),
        senior: list.leader.clone_astro(),
    }));
    // Update list
    list.leader = AstronautPointer::Astronaut(astronaut.clone());

    {
        let astronaut_ref = astronaut.borrow();
        println!("Succesfully created Captain {}!", astronaut_ref.name);

        match &astronaut_ref.senior {
            AstronautPointer::Astronaut(senior) => {
                println!("Leader: {:?}", senior.borrow().name);
            }
            _ => println!("No other astronaut"),
        }
    }
}

#[allow(dead_code)]
enum OptionSelector {
    Option1,
    Option2,
    Option3,
}
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
                let option_title: String = String::from("1. Create astronauts");
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
                let mut astronaut_list: Astronauts = create_astronaut_list();
                create_astronaut(&mut astronaut_list);
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

fn main() {
    // Options

    let options: Vec<&TerminalOption> = vec![unsafe { &*addr_of!(ASTRONAUT_LIST) } , &OPTION_2, &OPTION_3];
    // Astronaut linked-list

    // Selection algorithm
    let mut selection = String::new();
    loop {
        println!("Select an Option");
        println!("{}", options[0].present());
        println!("{}", options[1].present());
        println!("{}", options[2].present());

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        let selection: u32 = selection.trim().parse().expect("Not a number");

        println!("You selected {}", selection);

        match selection {
            1..=3 => {
                let n = (selection - 1) as usize;
                unsafe{
                    ASTRONAUT_LIST.list = options[n].execute();
                }
            }
            _ => {
                println!("Not a valid option")
            }
        }
    }
}
