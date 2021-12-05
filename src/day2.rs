use std::fs::File;

use serde::{Deserialize};

pub struct Submarine {
    pub aim : i32,
    pub depth : i32,
    pub horizontal_position: i32,
}
#[derive(Clone, Copy)]
pub enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}
pub type InputCmd = (String, i32);

impl Command {
    pub fn from_inputcmd(icmd: InputCmd) -> Command{
        match icmd.0.as_str() {
            "forward" => Command::Forward(icmd.1),
            "up" => Command::Up(icmd.1),
            "down" => Command::Down(icmd.1),
            &_ => panic!(),
        }
    }
}
impl Submarine {
    pub fn default() -> Submarine {
        Submarine{ aim: 0, depth: 0, horizontal_position: 0}
    }
    fn forward(&mut self, x: i32 ) {
        self.horizontal_position += x;
        self.depth += self.aim * x;
    }
    fn down(&mut self, x: i32) {
        self.aim += x; //If we go down we increase depth
    }
    fn up(&mut self, x : i32) {
        self.aim -= x; //If we go up we decrease depth
    }

    pub fn take_cmd(&mut self, command : Command) {
        match command {
            Command::Forward(input) => self.forward(input),
            Command::Up(input) => self.up(input), 
            Command::Down(input) => self.down(input), 
        }
    }
    pub fn take_cmd_list(&mut self, cmds: &[Command]){ 
        cmds.into_iter().for_each(|cmd| self.take_cmd(cmd.to_owned()));
    }
}

#[test]
fn cmd_test() {
    let mut sub : Submarine = Submarine::default();
    sub.take_cmd(Command::Forward(5));
    sub.take_cmd(Command::Down(5));
    sub.take_cmd(Command::Forward(8));
    sub.take_cmd(Command::Up(3));
    sub.take_cmd(Command::Down(8));
    sub.take_cmd(Command::Forward(2));
    assert_eq!(sub.depth, 60);
    assert_eq!(sub.horizontal_position, 15);
}


#[test]
fn cmd_list_test() {
    let mut sub : Submarine = Submarine::default();
    let cmd_list = vec![ Command::Forward(5)
            , Command::Down(5)
            , Command::Forward(8)
            , Command::Up(3)
            , Command::Down(8)
            , Command::Forward(2)
    ];
    sub.take_cmd_list(&cmd_list);
    assert_eq!(sub.depth, 60);
    assert_eq!(sub.horizontal_position, 15);
}

#[test]
fn day2_2_test() {
    let mut sub : Submarine = Submarine::default();
    let path = env!("CARGO_MANIFEST_DIR").to_string() + "/input/day2.csv";
    let file = File::open(path).unwrap();
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).delimiter(b' ').from_reader(file);
    let list : Result<Vec<InputCmd>, _ > = rdr.deserialize().collect();
    let cmdlist : Vec<Command> = list.unwrap().into_iter().map(|x| Command::from_inputcmd(x)).collect();
    sub.take_cmd_list(cmdlist.as_slice());
    let result = sub.depth * sub.horizontal_position;
    println!("Result of day 2.2 is {}", result);
    assert_eq!(result, 1176514794);
}
