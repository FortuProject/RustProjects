enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input{
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned())
    }
}

fn print_choice(choice: &Result<MenuChoice, String>) {
    //println!("choice = {:?}", choice);
}
pub fn main(){
    //let choice = get_choice("mainmenu");
    //print_choice(&choice);
   // match choice{
   //     Ok(choice) => print_choice(&choice),
   //     Err(e) => println!("error = {:?}", e),
   // }
}