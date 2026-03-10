use rand::Rng;
use std::io;
fn main() {
// Variables that Change.
let mut monster_name = String::new();
let mut player_name = String::new();
let mut input = String::new();

// Inputs from player
println!("enter thy name:");
    io::stdin().read_line(&mut player_name).unwrap();
    let player_name = player_name.trim().to_string();
println!("enter the monster's name:");
    io::stdin().read_line(&mut monster_name).unwrap();
    let monster_name = monster_name.trim().to_string();
println!("enter the monster's skill:");
    io::stdin().read_line(&mut input).unwrap(); //input/output and taking the "mut input"
    let monster_skill: i32 = input.trim().parse().unwrap(); // the input is assigned a name+number
    input.clear(); //clear the input to then resume aka borrowing and then cleaning

println!("enter the monster's stamina:");
    io::stdin().read_line(&mut input).unwrap();
    let mut monster_stamina: i32 = input.trim().parse().unwrap();
    input.clear();

println!("enter {}'s skill:", player_name);
    io::stdin().read_line(&mut input).unwrap();
    let player_skill: i32 = input.trim().parse().unwrap();
    input.clear();

println!("enter {}'s stamina:", player_name);
    io::stdin().read_line(&mut input).unwrap();
    let mut player_stamina: i32 = input.trim().parse().unwrap();
    input.clear();

println!("enter {}'s luck:", player_name);
    io::stdin().read_line(&mut input).unwrap();
    let mut player_luck: i32 = input.trim().parse().unwrap();
    input.clear();

// Preparing the Combat rounds
let mut rng = rand::thread_rng();
println!("------------------ \nThe battle begins! \n------------------");

// While loops during combat rounds
while monster_stamina > 0 && player_stamina > 0 {
    let monster_roll = rng.gen_range(1..=12);
    let player_roll = rng.gen_range(1..=12);
    let monster_attack = monster_skill + monster_roll;
    let player_attack = player_skill + player_roll;

    println!("Monster's attack: {}", monster_attack);// to check code is correct
    println!("Player's attack: {}", player_attack);// to check code is correct commit later

    if player_attack > monster_attack {
        let mut damage = 2;  // Base damage
            println!("{} endures injuries!", monster_name,);
            println!("Would thou like to test thy luck? (Y/N)");
                let mut choice = String::new();
                io::stdin().read_line(&mut choice).unwrap();
                let choice = choice.trim().to_lowercase();
                        if choice == "y" {
                            let luck_roll = rng.gen_range(1..=12);
                            println!("Luck roll: {}", luck_roll);
                        if player_luck >= luck_roll {
                            println!("Fortune smiles! Critical hit!");
                                damage = 4; // total damage becomes 4
                                                    } else {
                                                            println!("Your weapon deflects into the armour.");
                                                            damage = 1; // total damage becomes 1
                                                            }
                        player_luck -= 1; // luck decreases after testing
                        println!("{} remaining luck is {}", player_name, player_luck);
                                        }

        monster_stamina -= damage; // apply total damage at once
        println!("The {}'s remaining stamina: {}. n------------------", monster_name, monster_stamina);
                                    }

    else if monster_attack > player_attack {
        player_stamina -= 2;
        println!("{} endures injuries! \n------------------",player_name);
                                            }
    else {
        println!("Clash! Neither injured! \n------------------");
        }
        }
 
// If results from combat rounds
if player_stamina > 0 {
    println!("You breath out knowing, the {} is defeated!",monster_name);
                        }
else{
    println!("Darkness encloses, and the sound of a friend speaking reaches your ears. Death greets you.");
    }
}
