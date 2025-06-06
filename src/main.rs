use core::fmt;
use std::{cmp::min, io, thread, time};
use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};
use inline_colorization::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Suit {
    Red,
    Green,
    Black,
    Rose,
}

#[derive(Clone, Copy, Debug)]
struct Card {
    suit: Suit,
    value: i8,
}

impl Card {
    fn new(suit: Suit, value: i8) -> Self {
        return Card{suit, value}
    }
    fn stackable(lower: &Card, upper: &Card) -> bool {
        return (lower.suit != upper.suit) && (lower.value == (upper.value+1))
    }
    fn validstack(stack: &Vec<Card>) -> bool {
        if stack.len() <= 1 {return true};
        if stack.len() == 2 {return Card::stackable(&stack[0], &stack[1])}
        for i in 0..stack.len()-1 {
            if !Card::stackable(&stack[i], &stack[i+1]) {
                return false;
            }
        }
        return true;
    }
    fn is_dragon(&self) -> bool {
        return self.value == -1
    }
    fn is_rose(&self) -> bool {
        return self.suit == Suit::Rose
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String = String::new();
        match self.suit {
            Suit::Red => {s.push_str(color_red); s.push('R')},
            Suit::Green => {s.push_str(color_green); s.push('G')},
            Suit::Black => {s.push_str(color_bright_black); s.push('B')},
            Suit::Rose => {s.push_str(color_bright_magenta); s.push_str("Z")},
        }
        if self.value < 0 {
            s.push_str(color_reset);
            s.push('D');
        }
        if self.value > 0 {
            s.push_str(&self.value.to_string());
        }
        s.push_str(color_reset);
        write!(f, "{}", s).unwrap();
        Ok(())
    }
}

fn cardstack_tostring(stack: &Vec<Card>) -> String{
    if stack.is_empty() {return "[]".to_string()}
    let mut s: String = String::new();
    s.push('[');
    for card in stack {
        s.push_str(&format!("{}", card));
        s.push_str(", ");
    }
    s.pop(); s.pop();
    s.push(']');
    return s
}

fn print_help() {
    println!("How To Play:");
    println!("input a number to select a spot on the board");
    println!("if your hand is empty, you will grab from there");
    println!("else, you will attempt to place to there");
    println!();
    println!("Usual solitaire rules apply");
    println!("No stacking on same color/suit, only in descending order");
    println!("if 4 dragons of the same color are at the tops of stacks at the same time, ");
    println!("you can sacrifice a hold slot to store them away");
    println!("the rose card gets its own slot");
    println!("empty all 8 board slots to win");
    println!();
}

fn main() {

    'newgame: loop { // can you tell i program in assembly?

    println!("Input Seed Value (0 for random):");
    let mut seed: u64 = inputu64();
    if seed == 0 {
        seed = rand::rng().next_u64();
    }
    let seed: u64 = seed; // remove mutability

    'restartgame: loop {

    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    
    let mut recentgrab: usize = 0;
    let mut grabbed: Vec<Card> = Vec::new();
    let mut board: Vec<Vec<Card>> = Vec::new();
    for _ in 0..15 {
        board.push(Vec::new());
    }

    { // fill random game
        let mut ordered: Vec<Card> = Vec::new();                   // create an ordered deck
        for s in vec![Suit::Red, Suit::Green, Suit::Black] { // for each of the 3 suits
            for v in 1..=9 {                                   // and values 1-9
                ordered.push(Card::new(s.clone(), v)); // put them in the ordered stack
            }
            for _ in 0..4 {                                        // and do the same for the 4 dragons as well
                ordered.push(Card::new(s.clone(), -1));
            }
        }
        ordered.push(Card::new(Suit::Rose, -1));            // and finally the rose card

        let mut shuffled: Vec<Card> = Vec::new();                 
        for _ in 0..40 {                                          // create a shuffled deck by randomly removing then pushing
            let temp: Card = ordered.remove(rng.random_range(0..ordered.len()));
            shuffled.push(temp);
        }
        // println!("{}", cardstack_tostring(&shuffled));
        // distribute the cards across the board
        for i in 0..8 {
            println!("{}", cardstack_tostring(&board[i]));
        }
        println!();
        for _ in 0..5 {    // 5 cards per stack
            for width in 0..8 { // 8 stacks on the board
                board[width].push(shuffled.pop().unwrap());
                for i in 0..8 {
                    println!("{}", cardstack_tostring(&board[i])); // play a neat little animation for filling the board!
                }
                println!();
                thread::sleep(time::Duration::from_millis(50));
            }
        }

        for _ in 0..24 {println!()}
    }

    // { // fill almost completed game
    //     let mut slot: usize = 8;
    //     for s in vec![Suit::Red, Suit::Green, Suit::Black] {
    //         for _ in 0..4 {
    //             board[slot].push(Card::new(s, -1));
    //         }
    //         slot += 1;
    //     }
    //     for s in vec![Suit::Red, Suit::Green, Suit::Black] {
    //         for i in 1..=9 {
    //             board[slot].push(Card::new(s, i))
    //         }
    //         slot += 1;
    //     }
    //     board[14].push(Card::new(Suit::Rose, -1));

    //     let temp = board[11].pop().unwrap();
    //     board[0].push(temp);
    // }

    // { // fill known solveable game
    //     board[0].push(Card::new(Suit::Red, -1)); board[0].push(Card::new(Suit::Green, 4)); board[0].push(Card::new(Suit::Black, 5)); board[0].push(Card::new(Suit::Red, 7)); board[0].push(Card::new(Suit::Black, 6));
    //     board[1].push(Card::new(Suit::Black, -1));
    //     board[2].push(Card::new(Suit::Red, 2)); board[2].push(Card::new(Suit::Black, 9)); board[2].push(Card::new(Suit::Green, 3)); board[2].push(Card::new(Suit::Red, -1));
    //     board[3].push(Card::new(Suit::Red, 8));
    //     board[4].push(Card::new(Suit::Red, 9)); board[4].push(Card::new(Suit::Black, 8)); board[4].push(Card::new(Suit::Green, 7)); board[4].push(Card::new(Suit::Red, 6)); board[4].push(Card::new(Suit::Green, 5)); board[4].push(Card::new(Suit::Black, 4)); board[4].push(Card::new(Suit::Red, 3));
    //     board[5].push(Card::new(Suit::Red, -1)); board[5].push(Card::new(Suit::Green, 8)); board[5].push(Card::new(Suit::Black, -1));
    //     board[6].push(Card::new(Suit::Green, 9)); board[6].push(Card::new(Suit::Red, -1)); board[6].push(Card::new(Suit::Green, 2)); board[6].push(Card::new(Suit::Red, 5)); board[6].push(Card::new(Suit::Black, 7)); board[6].push(Card::new(Suit::Green, 6));
    //     board[7].push(Card::new(Suit::Black, -1)); board[7].push(Card::new(Suit::Red, 4));
    //     for _ in 0..4 {
    //         board[8].push(Card::new(Suit::Green, -1));
    //     }
    //     board[9].push(Card::new(Suit::Black, -1));
    //     board[11].push(Card::new(Suit::Red, 1));
    //     board[12].push(Card::new(Suit::Green, 1));
    //     for i in 1..=3 {
    //         board[13].push(Card::new(Suit::Black, i));
    //     }
    //     board[14].push(Card::new(Suit::Rose, -1));
    // }

    /*
    Board State:
    0: []
    1: [R4, G3]
    2: [R2, B9]
    3: [R8, G7, R6, G5, B4, R3]
    4: [R9, B8, R7, B6, R5, G4]
    5: []
    6: [G9]
    7: [G8, B7, G6, B5]
    8: hold1: [GD, GD, GD, GD]
    9: hold2: [BD, BD, BD, BD]
    10: hold3: [RD, RD, RD, RD]
    Final Stacks:
    11:   Red Cards: [R1]
    12: Green Cards: [G1, G2]
    13: Black Cards: [B1, B2, B3]
    14:   Rose Card: [ZD]
     */

    // { // progressed above state, 1 move away from win
    //     board[1].push(Card::new(Suit::Red, 4)); board[1].push(Card::new(Suit::Green, 3));
    //     board[2].push(Card::new(Suit::Red, 2)); board[2].push(Card::new(Suit::Black, 9));
    //     let mut val: i8 = 8;
    //     for suit in vec![Suit::Red, Suit::Green, Suit::Red, Suit::Green, Suit::Black, Suit::Red] {
    //         board[3].push(Card::new(suit, val));
    //         val -= 1;
    //     }
    //     let mut val: i8 = 9;
    //     for suit in vec![Suit::Red, Suit::Black, Suit::Red, Suit::Black, Suit::Red, Suit::Green] {
    //         board[4].push(Card::new(suit, val));
    //         val -= 1;
    //     }
    //     board[6].push(Card::new(Suit::Green, 9));
    //     let mut val: i8 = 8;
    //     for suit in vec![Suit::Green, Suit::Black, Suit::Green, Suit::Black] {
    //         board[7].push(Card::new(suit, val));
    //         val -= 1;
    //     }
    //     let mut slot: usize = 8;
    //     for suit in vec![Suit::Green, Suit::Black, Suit::Red] {
    //         for _ in 0..4 {
    //             board[slot].push(Card::new(suit, -1));
    //         }
    //         slot += 1;
    //     }
    //     board[11].push(Card::new(Suit::Red, 1));
    //     board[12].push(Card::new(Suit::Green, 1)); board[12].push(Card::new(Suit::Green, 2));
    //     board[13].push(Card::new(Suit::Black, 1)); board[13].push(Card::new(Suit::Black, 2)); board[13].push(Card::new(Suit::Black, 3)); 
    //     board[14].push(Card::new(Suit::Rose, -1));
    // }

    println!("Game Seed: {}", seed);
    println!();
    print_help();

    // main game loop
    loop {
        'stackloop: loop {
            // print gamestate
            println!("Recently Grabbed:");
            println!("{}", recentgrab);
            println!("Currently Grabbed:");
            println!("{}", cardstack_tostring(&grabbed));
            println!("Board State:");
            for i in 0..8 {
                println!(" {}: {}", i, cardstack_tostring(&board[i]));
            }
            println!(" 8: hold1: {}", cardstack_tostring(&board[8]));
            println!(" 9: hold2: {}", cardstack_tostring(&board[9]));
            println!("10: hold3: {}", cardstack_tostring(&board[10]));

            println!("Final Stacks:");
            println!("11:   Red: {}", cardstack_tostring(&board[11]));
            println!("12: Green: {}", cardstack_tostring(&board[12]));
            println!("13: Black: {}", cardstack_tostring(&board[13]));
            println!("14:  Rose: {}", cardstack_tostring(&board[14]));
            { // autostack, reprint gamestate if happens
                if grabbed.is_empty() {
                    let level: i8 = min(board[11].len(), min(board[12].len(), board[13].len())) as i8 + 1;
                    for i in 0..=10 {
                        if !board[i].is_empty() {
                            let c: &Card = board[i].last().unwrap();
                            if c.value == level || c.suit == Suit::Rose {
                                let slot: usize = match c.suit {
                                    Suit::Red => 11,
                                    Suit::Green => 12,
                                    Suit::Black => 13,
                                    Suit::Rose => 14,
                                };
                                let c: Card = board[i].pop().unwrap();
                                board[slot].push(c);
                                println!("Autostacked: {}", c);
                                println!();
                                thread::sleep(time::Duration::from_millis(50));
                                continue 'stackloop;
                            }
                        }
                    }
                }
                break 'stackloop;
            }
        }

        { // win check
            let mut flag: bool = true;
            if !grabbed.is_empty() {
                    flag = false;
                }
            for slot in 0..=7 {
                if !board[slot].is_empty() {
                    flag = false;
                }
            }
            if flag {
                println!("{}You Win!{}", color_bright_magenta, color_reset);
                return;
            }
        }

        println!("Other Moves:");
        println!("15: Stow Red Dragons");
        println!("16: Stow Green Dragons");
        println!("17: Stow Black Dragons");
        println!("18: Menu");

        println!("Make a move:");

        let mut successful: bool = false;
        // get player move
        let select: usize = input() as usize;
        if grabbed.is_empty() {
            match select {
                0..=7 => { // grab from board
                    if !board[select].is_empty() {
                        print!("[");
                        for i in 0..board[select].len()-1 {
                            if i < 10 {
                                print!(" ");
                            }
                            print!("{}, ", i);
                        }
                        if board[select].len()-1 < 10 {
                            print!(" ");
                        }
                        print!("{}", board[select].len()-1);
                        println!("]");
                    }
                    println!("{}", cardstack_tostring(&board[select]));
                    println!("Select index to grab from:");
                    let index: usize = input() as usize;
                    if index < board[select].len() {
                        let sublist: Vec<Card> = board[select].clone()[index..board[select].len()].to_vec();
                        if Card::validstack(&sublist) {
                            for c in sublist {
                                grabbed.push(c);
                                board[select].pop();
                            }
                            recentgrab = select;
                            successful = true;
                        }
                    }
                },
                8..=10 => { // grab from hold
                    if board[select].len() == 1 {
                        grabbed.push(board[select].pop().unwrap());
                        recentgrab = select;
                        successful = true;
                    }
                },
                11..=13 => { // grabb one card from Final stack
                    if !board[select].is_empty() {
                        grabbed.push(board[select].pop().unwrap());
                        recentgrab = select;
                        successful = true;
                    }
                },
                _ => (), // do nothing if no matches
            }
        } else {
            match select {
                0..=7 => { // place to board
                    let mut flag: bool = board[select].is_empty();
                    if select == recentgrab {flag = true}
                    if !flag {
                        flag = Card::stackable(&board[select].last().unwrap(), &grabbed[0]);
                    }
                    if flag {
                        while !grabbed.is_empty() {
                            board[select].push(grabbed.remove(0));
                        }
                        successful = true;
                    }
                },
                8..=10 => { // place to hold
                    if board[select].is_empty() && grabbed.len() == 1 {
                        board[select].push(grabbed.pop().unwrap());
                        successful = true;
                    }
                },
                11..=13 => { // placing one card to Final stack
                    let suit: Suit = match select {
                        11 => Suit::Red,
                        12 => Suit::Green,
                        13 => Suit::Black,
                        _ => Suit::Rose,
                    };
                    if grabbed.len() == 1 && grabbed[0].suit == suit {
                        if board[select].is_empty() && grabbed[0].value == 1 {
                            board[select].push(grabbed.pop().unwrap());
                            successful = true;
                        } else if board[select].last().unwrap().value+1 == grabbed[0].value {
                            board[select].push(grabbed.pop().unwrap());
                            successful = true;
                        }
                    }
                },
                14 => { // stow rose card
                if grabbed[0].is_rose() {
                    board[select].push(grabbed.pop().unwrap());
                    successful = true;
                }
            },
                _ => (), // do nothing if no matches
            }
        }
        
        match select {
            15..=17 => { // stow dragons 
                let suit: Suit = match select {
                    15 => Suit::Red,
                    16 => Suit::Green,
                    17 => Suit::Black,
                    _ => Suit::Rose,
                };
                let mut slot: usize = 0;
                for i in 8..=10 { // find a valid storage slot
                    if board[i].is_empty() {slot = i; break}
                    if board[i][0].suit == suit && board[i][0].is_dragon() {slot = i; break}
                }
                if slot != 0 {
                    let mut count: u8 = 0;
                    for i in 0..=10 {
                        if !board[i].is_empty() {
                            let c: &Card = board[i].last().unwrap();
                            if c.suit == suit && c.is_dragon() {
                                count += 1;
                            }
                        }
                    }
                    if count == 4 {
                        for i in 0..=10 {
                            if !board[i].is_empty() {
                                if board[i].last().unwrap().suit == suit && board[i].last().unwrap().is_dragon() {
                                    let temp: Card = board[i].pop().unwrap();
                                    board[slot].push(temp);
                                }
                            }
                        }
                        successful = true;
                    }
                }
            },
            18 => {
                successful = true;
                println!("0: Exit Menu");
                println!("1: Show Rules");
                println!("2: Restart Game");
                println!("3: New Game");
                println!("4: Quit Game");
                let menuselect: u8 = input();
                match menuselect {
                    1 => print_help(),
                    2 => continue 'restartgame,
                    3 => continue 'newgame,
                    4 => return,
                    _ => (),
                }
            }
            _ => (),
        }

        if !successful {
            println!("Invalid Move.");
        }

        for _ in 0..5 {println!()}
    }

    }
    }
}



fn input() -> u8 {
    loop {
        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        match value.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

fn inputu64() -> u64 {
    loop {
        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        match value.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}