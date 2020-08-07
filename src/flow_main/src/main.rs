extern crate flow_get_pos;
extern crate flow_command;

fn print_usage() {
	println!("--m : Get mouse position");
	println!("--r : Run commands trigger listener");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("No arguments");
		print_usage();
		return;
	}	
	
	match args[1].as_str() {
		"--m" => flow_get_pos::run(),
		"--r" => flow_command::run(),
		_ => print_usage(),
	}
}
