
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
