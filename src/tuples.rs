//tuples group together values of different types
//max 12 elements

pub fn run() {
    let pokemon: (&str, &str, i8) = ("Pikachu", "Electric", 25);
    println!("{}'s type is {}, Its pokedex number is {}.", pokemon.0, pokemon.1, pokemon.2);

}