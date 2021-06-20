//! Simple cli for interacting with human knowledge protocol
use hkp::network;
use dialoguer::{theme::ColorfulTheme, Select, Confirm};
use dialoguer::console::Term;

struct MenuItem {
    name: String,
    sub_menu: Option<Box<MenuItem>>,
}

struct Menu {
    menu_items: Vec<MenuItem>,
    has_back: Option<usize>,
    has_quit: Option<usize>,
    execute: Option<fn(menu: &Menu, item : usize)>,
}

impl Menu {
    fn new(items: Vec<&str>) -> Menu {
        let mut ret = Menu {menu_items : Vec::new(), has_back : None, has_quit : None, execute : Option::None};
        for i in items {
            let new_item = MenuItem{name: String::from(i) ,sub_menu: None};
            ret.menu_items.push(new_item);
        }
        return ret
    }

    fn as_vecstr(&self) -> Vec<&str> {
        let mut ret = Vec::new();
        for i in self.menu_items.iter() {
            ret.push(i.name.as_str());
        }
        return ret;
    }

    fn add_option_quit(&mut self) -> &mut Menu{
        self.menu_items.push(MenuItem{name: String::from("Quit"), sub_menu: None});
        self.has_quit = Some(self.menu_items.len()-1);
        return self;
    }

    fn run_option(&self, selection : usize) {
        println!("Exucuting option: {}: {}", selection, self.menu_items[selection].name);
        match self.execute {
            Some(x) => x(self, selection),
            None => println!("Menu has no function to execute...")
        }
    }
}

#[tokio::main]
async fn main() {
    
    println!("Human Knowledge Protocol CLI\n");

    let settings_menu = Menu::new(vec!["Author", "Connection"]);
    //settings_menu.add_option_back()
    let connection_menu = Menu::new(vec!["Select node", "Test Connection"]);
    //connection_menu.add_option_back()
    let mut menu = Menu::new(vec!["Search", "Write", "Settings"]);
    
    menu.add_option_quit();
    menu.execute = Some(main_menu_fn);
    
    let mut prompt_quit = false;

    loop { 
        let items = menu.as_vecstr();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(index) => {
                println!("User selected item : {}", items[index]);
                if index == menu.has_quit.unwrap() {
                    prompt_quit = true;
                }                
                else {
                    menu.run_option(index);
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

fn main_menu_fn(menu : &Menu, selection : usize) {
    match selection {
        0 => println!("Search the World!"),
        _ => println!("Unimplemented option selected...")
    }
}

/*async fn settings_menu(selection: Option<usize>, items: Vec<&str>) -> bool {
    match selection {
        Some(index) => {
            println!("User selected item : {}", items[index]);
            match index {
                0 => network::test_connection().await,
                2 => return true,
                _ => println!("Unimplemented option selected...")
                }
            }
        None => return false
    }
    return false
}*/