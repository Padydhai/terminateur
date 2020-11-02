
struct Robot {
    id: i32,
    pos_x: i32,
    pos_y: i32,
    orientation: Orientation,
    instructions: Vec<Instructions>,
}

#[derive(Debug)]
enum Orientation {
    North,
    East,
    South,
    West, 
}
#[derive(Debug)]
enum Instructions {
    F,
    L,
    R,
}

fn deplacement (mut bot: Robot)  {
    let vec = bot.instructions;
    for i in vec.into_iter() {
        match i {
            Instructions::F => match bot.orientation {
                Orientation::East  => bot.pos_x += 1,
                Orientation::North => bot.pos_y += 1,
                Orientation::South => bot.pos_y -= 1,
                Orientation::West  => bot.pos_x -= 1,
            }
            Instructions::L => match bot.orientation { 
                Orientation::East  => bot.orientation = Orientation::North,
                Orientation::North => bot.orientation = Orientation::West,
                Orientation::South => bot.orientation = Orientation::East,
                Orientation::West  => bot.orientation = Orientation::South,
            }
            Instructions::R => match bot.orientation {
                Orientation::East  => bot.orientation = Orientation::South,
                Orientation::North => bot.orientation = Orientation::East,
                Orientation::South => bot.orientation = Orientation::West,
                Orientation::West  => bot.orientation = Orientation::North,
            }
        } 
    }
}  



/*
fn deplacement (mut bot: Robot)  {
    let vec = bot.instructions;
    for i in vec.into_iter() {
        if i == Instructions::F {
            if bot.orientation ==  {
                bot.pos_y += 1
            } else if bot.orientation == "East" {
                bot.pos_x += 1
            } else if bot.orientation == "South" {
                bot.pos_y -= 1
            } else if bot.orientation == "West" {
                bot.pos_x -= 1
            }
        } else if i == 'L' {
            if bot.orientation == "North" {
                bot.orientation = String::from("West")
            } else if bot.orientation == "East" {
                bot.orientation = String::from("North")
            } else if bot.orientation == "South" {
                bot.orientation = String::from("East")
            } else if bot.orientation == "West" {
                bot.orientation = String::from("South")
            }
        } else if i == 'R' {
            if bot.orientation == "North" {
                bot.orientation = String::from("East")
            } else if bot.orientation == "East" {
                bot.orientation = String::from("South")
            } else if bot.orientation == "South" {
                bot.orientation = String::from("West")
            } else if bot.orientation == "West" {
                bot.orientation = String::from("North")
            }
        }
        
    }
    
}
*/


fn main() {


    let bot = Robot {
        id: 0,
        pos_x: 2,
        pos_y: 2,
        orientation: Orientation::South,
        instructions: vec![Instructions::F,Instructions::L,Instructions::L],
    };
    deplacement(&bot);
    println!("pos x et y {} {} et orientation {:?}", bot.pos_x, bot.pos_y, bot.orientation);

}