use rand::Rng;
use std::io;
use std::cmp::Ordering;

const OBSTACLE_DISPLAY: &str = "🔥";
const FREE_SPACE_DISPLAY: &str = "_";
const OBSTACLE: i8 = 1;
const FREE_SPACE: i8 = 0;

const COURSE_LEN: usize = 24;

fn main() {
    let course = make_course(COURSE_LEN);
    // the players place in the course
    let mut place: usize = 0;
    let mut first_roll = true;

    // pass in length of course to avoid printing out the current place of the player
    println!("board {:?}", make_board_display(&course.len(), &course));

    'game: loop {
        let mut roll = String::new();

        println!("🎲 Roll the dice…");

        io::stdin()
            .read_line(&mut roll)
            .expect("Failed to read line");

        let roll: usize = rand::thread_rng().gen_range(1..=6);

        println!("player rolled {}", roll);

        if place + roll >= course.len() {
            println!("🏆 Finished the course!");
            break 'game;
        } else if first_roll {
            place = roll - 1;
        } else {
            place += roll;
        }

        let space = course[place];

        if is_obstacle(&space) {
            println!("{} space is obstacle. go back 2 spaces", OBSTACLE_DISPLAY);
            place = hit_obstacle_next_place(place);
        } else {
            println!("✅ space is not obstacle.");
        }

        let snapshot = make_board_display(&place, &course);
        println!("board {:?}", snapshot);
        first_roll = false;
        println!("======");
    }
}

fn is_obstacle(space: &i8) -> bool {
    let obstacle_value: i8 = 1;
    *space == obstacle_value
}

fn make_board_display(place: &usize, course: &Vec<i8>) -> Vec<String> {
    let mut board: Vec<String> = vec![];
    for (i, &spot) in course.iter().enumerate() {
        if i == *place {
            board.push(format!("+{}", i));
        } else if spot == 1 {
            board.push(OBSTACLE_DISPLAY.to_string());
        } else {
            // safe
            board.push(FREE_SPACE_DISPLAY.to_string());
        }
    }
    board
}

fn make_course(len: usize) -> Vec<i8> {
    let mut i = 0;
    let mut course: Vec<i8> = vec![];
    while i < len {
        let space: i8 = rand::thread_rng().gen_range(0..=1);
        if space == 0 {
            course.push(FREE_SPACE);
        } else {
            course.push(OBSTACLE);
        }
        i += 1;
    }
    course
}

fn hit_obstacle_next_place(place: usize) -> usize {
    let tmp = place - 2;
    match tmp.cmp(&0) {
        Ordering::Equal => tmp,
        Ordering::Less => 0,
        Ordering::Greater => tmp,
    }
}