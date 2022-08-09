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
enum Flavor {
    Vanilla,
    Coconut,
    Lemon,
    Mint,
}

struct CocktailInfo {
    flavor: Flavor,
    fluid_once: i32,
}

fn print_drink_info(cocktail: CocktailInfo) {
    println!("{:?}", cocktail.fluid_once);
    match cocktail.flavor {
        Flavor::Vanilla => println!("Vanilla"),
        Flavor::Coconut => println!("Coconut"),
        Flavor::Lemon => println!("Lemon"),
        Flavor::Mint => println!("Mint"),
    }
}

fn main() {
    let vanilla_ice = CocktailInfo {
        flavor: Flavor::Vanilla,
        fluid_once: 33,
    };
    let coconut_dundy = CocktailInfo {
        flavor: Flavor::Coconut,
        fluid_once: 12,
    };
    let lemon_haze = CocktailInfo {
        flavor: Flavor::Lemon,
        fluid_once: 77,
    };
    let mojito = CocktailInfo {
        flavor: Flavor::Mint,
        fluid_once: 42,
    };

    print_drink_info(vanilla_ice);
    print_drink_info(coconut_dundy);
    print_drink_info(lemon_haze);
    print_drink_info(mojito);
}
