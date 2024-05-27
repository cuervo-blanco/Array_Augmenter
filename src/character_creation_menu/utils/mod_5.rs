
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
