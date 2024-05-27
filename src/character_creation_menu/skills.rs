
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

