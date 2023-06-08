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

enum DrinkFlavor {
  Sparkling,
  Sweet,
  Fruity
}

struct Drink {
  flavor: DrinkFlavor,
  oz: f64,
}

fn print_info(drink: Drink) {

  match drink.flavor {
    DrinkFlavor::Sparkling => println!("flavor: sparking"),
    DrinkFlavor::Sweet => println!("flavor: sweet"),
    DrinkFlavor::Fruity => println!("flavor: fruity"),
  }

  println!("oz: {:?}", drink.oz)
}

fn main() {
  let drink = Drink {
    flavor: DrinkFlavor::Sweet,
    oz: 16.0,
  };

  print_info(drink);


}
