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
        let monster_roll = rng.gen_range(2..=12);
        let player_roll = rng.gen_range(2..=12);
        let monster_attack = monster_skill + monster_roll;
        let player_attack = player_skill + player_roll;

        println!("{}'s attack: {}", monster_name, monster_attack);// to check code is correct
        println!("{}'s attack: {}", player_name, player_attack);// to check code is correct commit later

        if player_attack > monster_attack {
            let mut damage = 2;  // Base damage
            println!("{} quavers from the pain!", monster_name,);
            println!("Do you wish to tempt fate? (Y/N)");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim().to_lowercase();
            if choice == "y" {
                let luck_roll = rng.gen_range(2..=12);
                println!("Luck roll: {}", luck_roll);
                if player_luck >= luck_roll {
                    println!("*** Fortune smiles upon you! Critical hit! ***");
                    damage = 4; // total damage becomes 4
                } else {
                    println!("*** Your strike deflects into the armor. ***");
                    damage = 1; // total damage becomes 1
                }
                player_luck -= 1; // luck decreases after testing
                println!("*** {} remaining luck is {} ***", player_name, player_luck);
            }

            monster_stamina -= damage; // apply total damage at once
            println!("The {}'s remaining stamina is: {}. \n------------------", monster_name, monster_stamina);
        }

        else if monster_attack > player_attack {
            player_stamina -= 2;
            println!("{} staggers from injury! \n------------------",player_name);
        }
        else {
            println!("The blows collide! Neither is harmed!  \n------------------");
        }
    }

    // If results from combat rounds
    if player_stamina > 0 {
        println!("You exhale catching breath. With {} stamina left you vanquished the {}.", player_stamina, monster_name);
                        }
    else{
        println!("The darkness closes in, and a friend’s voice lingers in your ears as death welcomes you.");
    }
}
