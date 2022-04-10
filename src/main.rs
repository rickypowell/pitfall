use rand::Rng;
use std::io;
use std::cmp::Ordering;

const OBSTACLE_DISPLAY: &str = "🔥";
const FREE_SPACE_DISPLAY: &str = "_";

const COURSE_LEN: usize = 32;

fn main() {
    let course = make_course(COURSE_LEN);
    // the players place in the course
    let mut place: usize = 0;
    let mut first_roll = true;

    // pass in length of course to avoid printing out the current place of the player
    println!("🎬 {:?}", make_board_display(&course.len(), &course));

    'game: loop {
        // 1. roll the dice
        // 2. find the next place in the course for the player
        // 3. move the player to the next space
        // 4. print out the board

        let roll = roll_two_dice(1..7, 1..7);
        let roll = [roll.0, roll.1].iter().sum();

        let turn = PlayerTurn { roll, current_place: place };
        
        println!("⏭ {:?}", turn);

        let next_place = match find_next_place(&turn, COURSE_LEN, first_roll) {
            NextPlace::GameWon => {
                println!("🏆 Finished the course!");
                break 'game;
            },
            NextPlace::Place(p) => p,
        };

        place = move_player_to_next_place(&course, next_place);

        let snapshot = make_board_display(&place, &course);
        println!("🎯 {:?}", snapshot);
        first_roll = false;
        println!("======");
    }
}

fn make_board_display(place: &usize, course: &Vec<Space>) -> Vec<String> {
    let mut board: Vec<String> = vec![];
    for (i, &spot) in course.iter().enumerate() {
        let item = match spot {
            Space::Obstacle(_) => OBSTACLE_DISPLAY.to_string(),
            Space::FreeSpace => FREE_SPACE_DISPLAY.to_string(),
        };
        if i == *place {
            board.push(format!("+{}", i));
        } else {
            board.push(item);
        }
    }
    board
}

fn make_course(len: usize) -> Vec<Space> {
    let mut i = 0;
    let mut course: Vec<Space> = vec![];
    while i < len {
        let space: i8 = rand::thread_rng().gen_range(0..=1);
        if space == 0 {
            course.push(Space::FreeSpace);
        } else {
            let penalty: usize = rand::thread_rng().gen_range(2..4);
            course.push(Space::Obstacle(penalty));
        }
        i += 1;
    }
    course
}

fn hit_obstacle_next_place(place: usize, penalty: usize) -> usize {
    let tmp = place - penalty;
    match tmp.cmp(&0) {
        Ordering::Equal => tmp,
        Ordering::Less => 0,
        Ordering::Greater => tmp,
    }
}

///
/// Blocks for user input and rolls the die.
/// 
fn roll_two_dice(dice1: std::ops::Range<usize>, dice2: std::ops::Range<usize>) -> (usize, usize) {
    println!("🎲🎲 Roll two dice…");

    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");

    (rand::thread_rng().gen_range(dice1), rand::thread_rng().gen_range(dice2))
}

#[derive(Debug)]
struct PlayerTurn {
    roll: usize,
    current_place: usize,
}

enum NextPlace {
    Place(usize),
    GameWon,    
}

#[derive(Clone, Copy, Debug)]
enum Space {
    Obstacle(usize),
    FreeSpace,
}

fn find_next_place(turn: &PlayerTurn, course_len: usize, first_roll: bool) -> NextPlace {
    let next_place = turn.current_place + turn.roll;
    if next_place >= course_len {
        return NextPlace::GameWon;
    } else if first_roll {
        return NextPlace::Place(turn.roll - 1);
    } else {
        return NextPlace::Place(next_place);
    }
}

fn move_player_to_next_place(course: &Vec<Space>, next_place: usize) -> usize {

    let space = course.get(next_place);

    match space {
        Some(Space::Obstacle(v)) => {
            println!("{} space is obstacle. go back {} spaces", OBSTACLE_DISPLAY, v);
            return hit_obstacle_next_place(next_place, *v);
        },
        Some(Space::FreeSpace) => {
            println!("✅ space is not obstacle.");
            return next_place;
        },
        None => next_place,
    }
}
