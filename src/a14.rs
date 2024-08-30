// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:

struct PersonANC{
    age: i32,
    name: String,
    FavoriteColor: String,
}

fn print(data:&str){
    println!("{:?}", data);

}

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

pub fn main() {
    let people = vec![
        PersonANC{
            name: String::from("George"),
            FavoriteColor: String::from("green"),
            age: 18,
        },
        PersonANC{
            name: String::from("Anne"),
            FavoriteColor: String::from("blue"),
            age: 19,
        },
        PersonANC{
            name: String::from("Katie"),
            FavoriteColor: String::from("purple"),
            age: 20,
        },

    ];

    for person in people {
        if person.age <= 19 {
           print(&person.name);
            print(&person.FavoriteColor);

        }
    }


}
