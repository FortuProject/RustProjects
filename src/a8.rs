// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
pub fn main(){
    let sweet = Drink{
        flavor: flavor::Sweet,
        fluid_oz: 6.0,
    };
    print_drink(sweet);

    let bitter = Drink{
        flavor: flavor::Bitter,
        fluid_oz: 6.0,
    };

    print_drink(bitter);


}

fn print_drink(drink: Drink) {
    match drink.flavor{
        flavor::Sweet => println!("sweet"),
        flavor::Sour => println!("Sour"),
        flavor::Bitter => println!("Bitter"),
    }

    println!("oz: {:?}", drink.fluid_oz)
}

enum flavor{
    Sweet,
    Sour,
    Bitter,
}

struct Drink {
    flavor: flavor,
    fluid_oz: f64,
}