// that can also work like a stack with a pop and push capabilities
// which we will call differently.
// we must also perform search of that array to find a particular name
mod astronaut;

use std::cell::RefCell;
use std::fmt;
use std::thread;
use std::ptr::addr_of;
use std::io;
use std::rc::Rc;

use core::time::Duration;
// External Crates
use once_cell::sync::Lazy;


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
