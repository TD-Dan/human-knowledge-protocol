//! Simple cli for interacting with human knowledge protocol
use hkp::network;
use dialoguer::{theme::ColorfulTheme, Select, Confirm};
use dialoguer::console::Term;

#[tokio::main]
async fn main() {
    
    println!("Human Knowledge Protocol CLI\n");

    
    println!("Select option:\n");
    let mut prompt_quit = false;
    let items = vec!["Test Connection", "Quit"];

    loop { 
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(index) => {
                println!("User selected item : {}", items[index]);
                match index {
                    0 => network::test_connection().await,
                    1 => prompt_quit = true,
                    _ => println!("Unimplemented option selected...")
                    }
                }
            None => prompt_quit = true
        }

        if prompt_quit {
            if Confirm::new().with_prompt("Quit?").interact().unwrap() {
                break;
            }
            prompt_quit = false;
        }

    }

}
