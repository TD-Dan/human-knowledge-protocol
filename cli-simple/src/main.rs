//! Simple cli for interacting with human knowledge protocol
use hkp::network;
use dialoguer::{theme::ColorfulTheme, Select, Confirm};
use dialoguer::console::Term;

struct MenuItem {
    name: String,
}

struct Menu{
    menu_items: Vec<MenuItem>,
    has_back: Option<usize>,
    has_quit: Option<usize>,
    execute: Option<fn(item : usize) -> Option<Menu>>,
}

impl Menu {
    fn new(items: Vec<&str>) -> Menu {
        let mut ret = Menu {menu_items : Vec::new(), has_back : None, has_quit : None, execute : Option::None};
        for i in items {
            let new_item = MenuItem{name: String::from(i)};
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
        self.menu_items.push(MenuItem{name: String::from("Quit")});
        self.has_quit = Some(self.menu_items.len()-1);
        return self;
    }

    fn add_option_back(&mut self) -> &mut Menu{
        self.menu_items.push(MenuItem{name: String::from("Back")});
        self.has_back = Some(self.menu_items.len()-1);
        return self;
    }

    fn run_option(&self, selection : usize) -> Option<Menu>{
        println!("Exucuting option: {}: {}", selection, self.menu_items[selection].name);
        match self.execute {
            Some(fnx) => {
                return fnx(selection);
            }
            None => println!("Menu has no function to execute...")
        }
        return None;
    }
}

#[tokio::main]
async fn main() {
    
    println!("Human Knowledge Protocol CLI\n");

    let connection_menu = Menu::new(vec!["Select node", "Test Connection"]);
    //connection_menu.add_option_back()
    let mut main_menu = Menu::new(vec!["Search", "Write", "Settings"]);
    
    main_menu.add_option_quit();
    main_menu.execute = Some(main_menu_fn);
    
    let mut menu = main_menu;
    let mut prev_menu = None;

    loop { 
        let items = menu.as_vecstr();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        let mut prompt_quit = false;

        match selection {
            Some(index) => {
                println!("User selected item : {}", items[index]);
                if menu.has_quit.is_some() {
                    if index == menu.has_quit.unwrap() {
                        prompt_quit = true;
                    }
                }           
                if menu.has_back.is_some() {
                    if index == menu.has_back.unwrap() {
                        if prev_menu.is_some() {
                            println!("Backing up");
                            menu = prev_menu.unwrap();
                            prev_menu = None
                        }
                    }
                }     
                if !prompt_quit {
                    match menu.run_option(index) {
                        Some(x) => {
                            prev_menu = Some(menu);
                            menu = x;
                        }
                        None => continue,
                    }
                }
            }
            None => prompt_quit = true
        }

        if prompt_quit {
            if Confirm::new().with_prompt("Quit?").interact().unwrap() {
                break;
            }
        }

    }

}

fn main_menu_fn(selection : usize) -> Option<Menu> {
    match selection {
        0 => println!("Search the World!"),
        2 => {
            let mut settings_menu = Menu::new(vec!["Author", "Connection"]);
            settings_menu.execute = Some(settings_menu_fn);
            settings_menu.add_option_back();
            return Some(settings_menu);
        }
        _ => println!("Unimplemented option selected...")
    }
    return None;
}

fn settings_menu_fn(selection : usize) -> Option<Menu> {
    match selection {
        //0 => network::test_connection().await,
        2 => return None,
        _ => println!("Unimplemented option selected...")
    }
    return None
}