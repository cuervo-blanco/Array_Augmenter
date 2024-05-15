//I want to be able to add names to an array through linked lists
// That can also work like a stack with a pop and push capabilities
// which we will call differently.
// we must also perform search of that array to find a particular name

use std::io;
use std::rc::Rc;
use std::cell::RefCell;

enum AstronautPointer {
    Astronaut(Rc<RefCell<Astronaut>>),
    Hat(Rc<RefCell<Hat>>),
}

impl AstronautPointer {
    fn clone_astro(&self) -> AstronautPointer {
        match self {
            AstronautPointer::Astronaut(astronaut) => {
                AstronautPointer::Astronaut(astronaut.clone())
            },
            AstronautPointer::Hat(hat) => {
                AstronautPointer::Hat(hat.clone())
            }
        }
    }
}

struct Characters {
    leader: AstronautPointer,
}

#[allow(dead_code)]
struct Hat {
    item: String,
}

struct Astronaut {
    name: String,
    senior: AstronautPointer,
}

#[allow(dead_code)]
fn create_character_list() -> Characters {
    // Ask more information about what to add and then kick
    // other functions based on that.
    println!("Starting creation process...");

    // Captain hat creation
    let captain_hat: Rc<RefCell<Hat>> = Rc::new(RefCell::new(Hat { item: "Captain's Hat!".to_string()}));
    //Characters list initiation process
    let character_list: Characters = Characters{
        leader: AstronautPointer::Hat(captain_hat),
    };

    character_list
}

fn create_character(list: &mut Characters) -> &mut Characters {
    // Get User Input for name
    println!("Enter Austronaut name; try to be nice to him: ");
    let mut name: String = String::new();
    let _ = io::stdin().read_line(&mut name);
    // Create Astronaut
    let astronaut: Rc<RefCell<Astronaut>> = Rc::new(RefCell::new(Astronaut{
        name: name.trim().to_string(),
        senior: list.leader.clone_astro(),
    }));
    // Update list
    list.leader =  AstronautPointer::Astronaut(astronaut.clone());

    {
        let astronaut_ref = astronaut.borrow();
        println!("Succesfully created Captain {}!", astronaut_ref.name);

        match &astronaut_ref.senior {
            AstronautPointer::Astronaut(senior) => {
                println!("Leader: {:?}", senior.borrow().name);
            },
            _ => println!("No other astronaut"),
        }
    }
    list
}

#[allow(dead_code)]
enum OptionSelector {
    Characters,
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
    fn present(&self) -> String {
        match self.func {
            OptionSelector::Characters => {
                // Should output the first structure in an array (which is the option)
                // The option then
                let option_title: String = String::from("1. Create characters");
                option_title
            },
            OptionSelector::Option2 => {
                // Should output the first structure in an array (which is the option)
                // The option then
                let option_title: String = String::from("2. Empty");
                option_title
            },
            _ => {
                let option_title: String = String::from("3. Empty");
                option_title
            }

    }
}
    fn execute(&self) {
        match self.func {
            OptionSelector::Characters => {
                // Should output the first structure in an array (which is the option)
                // The option then
                let mut character_list = create_character_list();
                create_character(&mut character_list);
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
   let characters = TerminalOption {
        id: 1,
        func: OptionSelector::Characters,
    };

    let option2 = TerminalOption {
        id: 2,
        func: OptionSelector::Option2,
    };

    let option3 = TerminalOption {
        id: 3,
        func: OptionSelector::Option3
    };

    let options: Vec<TerminalOption> = vec![characters, option2, option3];
    // Astronaut linked-list


    // Selection algorithm
    let mut selection = String::new();

    loop {
        println!("Select an Option");
        println!("{}", options[0].present());
        println!("{}", options[1].present());
        println!("{}", options[2].present());

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
