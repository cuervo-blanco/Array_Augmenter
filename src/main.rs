// That can also work like a stack with a pop and push capabilities
// which we will call differently.
// we must also perform search of that array to find a particular name

use std::cell::RefCell;
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
struct Astronaut {
    name: String,
    senior: Leader
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

fn main() {
    // Options

    let options: Vec<&TerminalOption> = vec![unsafe { &*addr_of!(ASTRONAUT_LIST) } , &OPTION_2, &OPTION_3];
    // Astronaut linked-list

    // Selection algorithm
    loop {

        unsafe {
            println!("Current list: {:?}", ASTRONAUT_LIST.list);
        }
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
