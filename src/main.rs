use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Robot {
    pos_x: i32,
    pos_y: i32,
    orientation: Orientation,
    instructions: Vec<Instructions>,
}

#[derive(Debug, PartialEq)]
enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
enum Instructions {
    F,
    L,
    R,
}

impl std::str::FromStr for Orientation {
    type Err = String;

    fn from_str(orien_str: &str) -> Result<Self, Self::Err> {
        match orien_str {
            "N" => Ok(Orientation::North),
            "E" => Ok(Orientation::East),
            "S" => Ok(Orientation::South),
            "W" => Ok(Orientation::West),
            _ => Err(format!("Tourner dans le vide vide")),
        }
    }
}

fn from_char(instru_char: char) -> Result<Instructions, String> {
    match instru_char {
        'F' => Ok(Instructions::F),
        'L' => Ok(Instructions::L),
        'R' => Ok(Instructions::R),
        _ => Err(format!("TAISEZ-VOUS VOus vous")),
    }
}

impl Orientation {
    fn rotate_left(self) -> Orientation {
        match self {
            Orientation::East => Orientation::North,
            Orientation::North => Orientation::West,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
        }
    }

    fn rotate_right(self) -> Orientation {
        match self {
            Orientation::East => Orientation::South,
            Orientation::North => Orientation::East,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }
}

impl Robot {
    fn forward(mut bot: Self) {
        match bot.orientation {
            Orientation::East => bot.pos_x += 1,
            Orientation::North => bot.pos_y += 1,
            Orientation::South => bot.pos_y -= 1,
            Orientation::West => bot.pos_x -= 1,
        }
    }
}

fn za_warudo(x_max: i32, y_max: i32, terminateurs: Vec<Robot>) {
    println!("Terrain [ x_max = {}; y_max = {} ]", x_max, y_max);
}

fn main() -> std::io::Result<()> {
    // Lecture du file.txt
    // Dédicace au groupe de Claire et Thomas pou vous avoir aider (Clarification du comment du pourquoi)
    let file = File::open("./two_robots.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut infos = contents.split_whitespace();

    let map_x = infos.next().unwrap().parse::<i32>().unwrap();
    let map_y = infos.next().unwrap().parse::<i32>().unwrap();
    let mut lairobeau: Vec<Robot> = Vec::new();

    loop {
        let mut bot = Robot {
            pos_x: infos.next().unwrap().parse::<i32>().unwrap(),
            pos_y: infos.next().unwrap().parse::<i32>().unwrap(),
            orientation: Orientation::from_str(infos.next().unwrap()).unwrap(),
            instructions: Vec::new(),
        };

        for instr in infos.next().unwrap().chars() {
            bot.instructions.push(from_char(instr).unwrap());
        }
        lairobeau.push(bot);
        if infos.next() == None {
            break;
        }
    }
    za_warudo(map_x, map_y, lairobeau);
    // println!("{:?}", lairobeau);
    Ok(())
}
