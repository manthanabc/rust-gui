use pixel_sdl2::*;

fn main() {
	let mut draw = Draw::new(400, 400);
	draw.stroke(Color::RGB(255, 255, 255));
	draw.rect(0, 0, 200, 200);

	'draw: loop {
		draw.line(0, 0, 400, 400);

		draw.handel_window_events();
	}
}