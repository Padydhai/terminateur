use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use strum_macros::EnumString;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Robot {
    pos_x: i32,
    pos_y: i32,
    orientation: Orientation,
    instructions: String,
}

#[derive(Debug, PartialEq, EnumString)]
enum Orientation {
    #[strum(serialize = "N")]
    North, 
    #[strum(serialize = "E")]
    East, 
    #[strum(serialize = "S")]
    South, 
    #[strum(serialize = "W")]
    West, 
}

#[derive(Debug, PartialEq, EnumString)]
enum Instructions {
    #[strum(serialize = "F")]
    Forward, 
    #[strum(serialize = "L")]
    Left, 
    #[strum(serialize = "R")]
    Right,
}


/*

// FONCTION DEPLACEMENT

impl Orientation {
    pub fn rotate_left(self) -> Orientation {
        match self {
            Orientation::East  => Orientation::North,
            Orientation::North => Orientation::West,
            Orientation::South => Orientation::East,
            Orientation::West  => Orientation::South,
        }
    }

    fn rotate_right(self) -> Orientation {
        match self {
            Orientation::East  => Orientation::South,
            Orientation::North => Orientation::East,
            Orientation::South => Orientation::West,
            Orientation::West  => Orientation::North,
        }
    }

}*/

/*
fn indila(bot : &Robot) {
    let vec = &bot.instructions;
    for i in vec.into_iter(){
        if i == &Instructions::L {
            Orientation::rotate_left(bot.orientation);
        } else if i == &Instructions::R {
            Orientation::rotate_right(bot.orientation);
        } else if i == &Instructions::F {
            forward(bot)
        }
            
        unimplemented!();
    }
}
*/
/*fn forward(bot : &Robot) {
    match bot.orientation {
        Orientation::East  => bot.pos_x += 1,
        Orientation::North => bot.pos_y += 1,
        Orientation::South => bot.pos_y -= 1,
        Orientation::West  => bot.pos_x -= 1,
    }
    unimplemented!();
}*/




/*fn deplacement (mut bot: &Robot)  {
    let vec = &bot.instructions;
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
}  */


fn main() -> std::io::Result<()> {
    
    // Lecture du file.txt 
    // DÉDICACE AU GROUPE DE CLAIRE ET THOMAS POUR NOUS AVOIR AIDER 
    let file = File::open("two_robots.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut iter: Vec<_> = contents.split_whitespace().collect::<Vec<_>>();

   // let map_x = iter.remove(0).parse::<i32>().unwrap();
   // let map_y = iter.remove(0).parse::<i32>().unwrap();
    
    let mut lairobeau: Vec<Robot> = Vec::new();
    while iter.len() > 0 {
        let bot = Robot {
            pos_x: iter.remove(0).parse::<i32>().unwrap(),
            pos_y: iter.remove(0).parse::<i32>().unwrap(),
            orientation: iter.remove(0).Orientation::from_str("N"|"S"|"W"|"E").unwrap(),
            instructions: iter.remove(0).to_string(),
        };
        lairobeau.push(bot);
    }
    println!("{:?}", lairobeau);


    
   /* let bot = Robot {
        pos_x: 2,
        pos_y: 2,
        orientation: Orientation::South,
        instructions: vec![Instructions::F,Instructions::L,Instructions::L],
    };
    indila(&bot);
    println!("pos x et y {} {} et orientation {:?}", bot.pos_x, bot.pos_y, bot.orientation);*/
    Ok(())
}
