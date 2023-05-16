use rand::Rng;
use std::io;

const INVALID_DOOR: i32 = 0;
const DOOR_1: i32 = 1;
const DOOR_2: i32 = 2;
const DOOR_3: i32 = 3;

fn main() {
    loop {
        game_loop();
    }
}

fn game_loop() {
    let prize_door = get_prize_door();
    let choice_door = get_choice_door();
    let (reveal_door, switch_door) = match prize_door == choice_door {
        true => get_reveal_and_switch_doors_when_choice_and_prize_are_same(prize_door, choice_door),
        _ => get_reveal_and_switch_doors_when_choice_and_prize_are_not_same(prize_door, choice_door)
    };

    println!("choice is {choice_door}, reveal is {reveal_door}, press 1 to switch:");

    let mut choice_switch_stay: String = String::new();

    io::stdin()
        .read_line(&mut choice_switch_stay)
        .expect("Failed to read line");

    let choice_door = match choice_switch_stay.trim() {
        "1" => switch_door,
        _ => choice_door,
    };

    match choice_door == prize_door {
        true => println!("You win the prize! 🏎️"),
        false => println!("You lose! 🐐"),
    }
}

fn get_reveal_and_switch_doors_when_choice_and_prize_are_not_same(
    prize_door: i32,
    choice_door: i32,
) -> (i32, i32) {
    let (reveal_door, switch_door) = match (prize_door, choice_door) {
        (DOOR_1, DOOR_2) => (DOOR_3, DOOR_1),
        (DOOR_1, DOOR_3) => (DOOR_2, DOOR_1),
        (DOOR_2, DOOR_1) => (DOOR_3, DOOR_2),
        (DOOR_2, DOOR_3) => (DOOR_1, DOOR_2),
        (DOOR_3, DOOR_1) => (DOOR_2, DOOR_3),
        (DOOR_3, DOOR_2) => (DOOR_1, DOOR_3),
        _ => (INVALID_DOOR, INVALID_DOOR),
    };

    (reveal_door, switch_door)
}

fn get_reveal_and_switch_doors_when_choice_and_prize_are_same(
    prize_door: i32,
    choice_door: i32,
) -> (i32, i32) {
    let random_door = loop {
        let random_door = rand::thread_rng().gen_range(0..=1);

        if random_door == choice_door || random_door == prize_door {
            continue;
        }

        break random_door;
    };

    let (reveal_door, switch_door) = match (prize_door, random_door) {
        (DOOR_1, 0) => (DOOR_2, DOOR_3),
        (DOOR_1, 1) => (DOOR_3, DOOR_2),
        (DOOR_2, 0) => (DOOR_1, DOOR_3),
        (DOOR_2, 1) => (DOOR_3, DOOR_1),
        (DOOR_3, 0) => (DOOR_1, DOOR_2),
        (DOOR_3, 1) => (DOOR_2, DOOR_1),
        _ => (INVALID_DOOR, INVALID_DOOR),
    };

    (reveal_door, switch_door)
}

fn get_prize_door() -> i32 {
    rand::thread_rng().gen_range(1..=3)
}

fn get_choice_door() -> i32 {
    println!("Choose door 1, 2, or 3.");

    let mut choice_door: String = String::new();

    io::stdin()
        .read_line(&mut choice_door)
        .expect("Failed to read line");

    let choice_door: i32 = choice_door
        .trim()
        .parse()
        .expect("failed to parse choice door");

    choice_door
}
