
pub fn get_config() -> &'static global::Global<flow_command_json::Config> {
	
	static CONFIG: global::Global<flow_command_json::Config> = global::Global::new();
	return &CONFIG;
}
