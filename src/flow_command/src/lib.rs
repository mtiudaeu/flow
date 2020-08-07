extern crate serde;
extern crate serde_json;

extern crate global;

extern crate flow_command_json;

extern crate inputbot;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

mod config_instance;

fn read_config_from_file(file_name: &str) -> Result<String, io::Error> {
    let mut f = File::open(file_name)?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn handle_command(command : &flow_command_json::Command) {
    //mdtmp
    println!("Before Lock");
    //mdtmp this cause a bug
	//mdtmp let command_interval_ms = config_instance::get_config().lock().unwrap().command_interval_ms;
    //mdtmp let wait_time = std::time::Duration::from_millis(command_interval_ms);
    let wait_time = std::time::Duration::from_millis(50);
    println!("After Lock");

    command.command_values.iter().for_each(|command_value| {
        match command_value.command_type {
            flow_command_json::CommandType::MoveAbs => command_run_move_abs(command_value),
            flow_command_json::CommandType::MoveRel => command_run_move_rel(command_value),
            flow_command_json::CommandType::Press => command_run_press(command_value),
            flow_command_json::CommandType::Release => command_run_release(command_value),
        }

		std::thread::sleep(wait_time);
    });
}
fn command_run_move_abs(command_value : &flow_command_json::CommandValue) {
    match &command_value.point {
        Some(p) => inputbot::MouseCursor::move_abs(inputbot::MouseCursor, p.x, p.y),
        None    => println!("command_run_move_abs : Point doesn't exist, cannot move."),
    }
}
fn command_run_move_rel(command_value : &flow_command_json::CommandValue) {
    match &command_value.point {
        Some(p) => inputbot::MouseCursor::move_rel(inputbot::MouseCursor, p.x, p.y),
        None    => println!("command_run_move_rel : Point doesn't exist, cannot move."),
    }
}
fn command_run_press(command_value : &flow_command_json::CommandValue) {
    if is_mouse_button(&command_value.key) {
        get_inputbot_mouse_key(&command_value.key).press();
    } else {
        get_inputbot_keyboard_key(&command_value.key).press();
    }
}
fn command_run_release(command_value : &flow_command_json::CommandValue) {
    if is_mouse_button(&command_value.key) {
        get_inputbot_mouse_key(&command_value.key).release();
    } else {
        get_inputbot_keyboard_key(&command_value.key).release();
    }
}

fn is_mouse_button(key : &flow_command_json::Key) -> bool {
    match key {
        flow_command_json::Key::Mouse | flow_command_json::Key::MouseLeft | flow_command_json::Key::MouseRight => return true,
        _ => return false,
    }
}

fn get_inputbot_mouse_key(key : &flow_command_json::Key) -> inputbot::MouseButton {
    match key {
        flow_command_json::Key::MouseLeft => return inputbot::MouseButton::LeftButton,
        flow_command_json::Key::MouseRight => return inputbot::MouseButton::RightButton,
        _ => {
            println!("get_inputbot_mouse_key : Error");
            return inputbot::MouseButton::RightButton;
        },
    }
}
fn get_inputbot_keyboard_key(key : &flow_command_json::Key) -> inputbot::KeybdKey {
    match key {
        flow_command_json::Key::A => return inputbot::KeybdKey::AKey,
        flow_command_json::Key::B => return inputbot::KeybdKey::BKey,
        flow_command_json::Key::C => return inputbot::KeybdKey::CKey,
        flow_command_json::Key::D => return inputbot::KeybdKey::DKey,
        flow_command_json::Key::E => return inputbot::KeybdKey::EKey,
        _ => {
            println!("get_inputbot_keyboard_key : Error");
            return inputbot::KeybdKey::AKey;
        },
    }
}


pub fn run() {
	
    let config_filename = "config.json";
    let update_config = || {
        let config_str = match read_config_from_file(config_filename) {
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
            Ok(config_str) => config_str,
        };

        let res = serde_json::from_str::<flow_command_json::Config>(&config_str.to_string());
        if !res.is_ok() {
            println!("Failed to parse Json");
            return;
        }

        let config: flow_command_json::Config = res.unwrap();
		*config_instance::get_config().lock_mut().unwrap() = config;

    };
	update_config();

	let config  = &*config_instance::get_config().lock().unwrap();

    let mut command_map : HashMap<String, flow_command_json::Command> = HashMap::new();

    config.commands.iter().for_each(|command| {
        command_map.insert(command.command_name.clone(), command.clone());
    });

    config.triggers.clone().iter().for_each(|trigger| {
        let command_option = command_map.get(&trigger.command_name);
        match command_option {
            Some(command) => {
                let command_clone = command.clone();
                if is_mouse_button(&trigger.key) {
                    get_inputbot_mouse_key(&trigger.key).bind(move || {
                        handle_command(&command_clone);
                    });
                } else {
                    get_inputbot_keyboard_key(&trigger.key).bind(move || {
                        handle_command(&command_clone);
                    });
                }
            },
            None    => println!("Initialize triggers : Command not found"),
        }
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}
