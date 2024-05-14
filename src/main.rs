//I want to be able to add names to an array through linked lists
// That can also work like a stack with a pop and push capabilities
// which we will call differently.
// we must also perform search of that array to find a particular name

use std::io;
use std::rc::Rc;
use std::cell::RefCell;

#[allow(dead_code)]
struct Astronaut {
    name: String,
    captain: bool,
    prev: Option<Rc<RefCell<Astronaut>>>,
    next: Option<Rc<RefCell<Astronaut>>>,
}

impl Astronaut{
    fn new( name: String,
            captain: bool,
            _prev: Option<Rc<RefCell<Astronaut>>>,
            _next: Option<Rc<RefCell<Astronaut>>>) -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(Astronaut {
                                                name,
                                                captain,
                                                prev: None,
                                                next: None
                                                }
                                    )
                        )
            }
}

#[allow(dead_code)]
fn add() -> Rc<RefCell<Astronaut>>{
    // Ask more information about what to add and then kick
    // other functions based on that.
    println!("Starting creation process...");
    println!("Enter Austronaut name; try to be nice to him: ");
    let mut name: String = String::new();
    let _ = io::stdin().read_line(&mut name);
    let captain_hat: Option<Rc<RefCell<Astronaut>>> = None;
    let captain = Astronaut::new(
        name.trim().to_string(),
        true,
        captain_hat,
        None::<Rc<RefCell<Astronaut>>>
    );
    {
        let captain_ref = captain.borrow();
        println!("Succesfully created Captain {}!", captain_ref.name);
    }
    captain
}

#[allow(dead_code)]
enum OptionSelector {
    Option1,
    Option2,
    Option3
}
#[allow(dead_code)]
struct TerminalOption {
    id: u32,
    func: OptionSelector,
}

#[allow(dead_code)]
impl TerminalOption{
    fn execute(&self) {
        match self.func {
            OptionSelector::Option1 => {
                // Should output the first structure in an array (which is the option)
                // The option then
                println!("You selected option 1");
                add();
            },
            OptionSelector::Option2 => {
                // Should output the first structure in an array (which is the option)
                // The option then
                println!("You selected option 2");
            },
            _ => {
                println!("You selected another option");
            }
        }
    }
}

fn main() {
    // Options
   let option1 = TerminalOption {
        id: 1,
        func: OptionSelector::Option1,
    };

    let option2 = TerminalOption {
        id: 2,
        func: OptionSelector::Option2,
    };

    let option3 = TerminalOption {
        id: 3,
        func: OptionSelector::Option3
    };

    let options: Vec<TerminalOption> = vec![option1, option2, option3];
    // Astronaut linked-list


    // Selection algorithm
    let mut selection = String::new();

    loop {
        println!("Select an Option");

        io::stdin().read_line(&mut selection).expect("Failed to read line");

        let selection: u32 = selection.trim().parse().expect("Not a number");

        println!("You selected {}", selection);


        match selection {
            1..=3 => {
                let n = (selection - 1) as usize;
                options[n].execute();
            },
            _ => {  println!("Not a valid option") }
        }
    }


}
