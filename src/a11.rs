// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

pub fn main() {
    let my_item = &grocery_item {
        quantity: 3,
        id_num: 99,
    };
    display(my_item);
    display_id(my_item);

}

fn display(item: &grocery_item){
    println!("quantity: {:?}", item.quantity);

}

struct grocery_item{
    quantity: i32,
    id_num: i32,
}

fn display_id(item: &grocery_item){
    println!("quantity: {:?}", item.id_num);

}