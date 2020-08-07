extern crate winapi;

use winapi::shared::windef::POINT;

fn get_mouse_pos() -> (i32, i32) {
    let mut point = POINT { x: 0, y: 0 };
    unsafe { ::winapi::um::winuser::GetCursorPos(&mut point as *mut POINT) };
    (point.x, point.y)
}
pub fn run() {
    loop {
        let move_wait_time = std::time::Duration::from_millis(500);
        std::thread::sleep(move_wait_time);
		let (x,y) = get_mouse_pos();
		println!("{}, {}", x, y);
    }
}
