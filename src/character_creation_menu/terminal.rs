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
