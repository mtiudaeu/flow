use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Key {
    Mouse,
    MouseRight,
    MouseLeft,
    A,
    B,
    C,
    D,
    E,
    //mdtmp
}
impl Default for Key {
    fn default() -> Self { Key::Mouse } // serde need a Default
}


#[derive(Serialize, Deserialize, Clone)]
pub enum CommandType {
    MoveAbs,
    MoveRel,
    Press,
    Release,
}
impl Default for CommandType {
    fn default() -> Self { CommandType::MoveAbs } // serde need a Default
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CommandValue {
    pub key : Key,
	pub command_type : CommandType,
	pub point: Option<Point>
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Point {
	pub x: i32,
	pub y: i32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Command {
	pub command_name : String,
	pub command_values : Vec<CommandValue>
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Trigger {
	pub key: Key,
	pub command_name: String
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
	pub commands : Vec<Command>,
	pub triggers : Vec<Trigger>,
	pub command_interval_ms: u64
}
