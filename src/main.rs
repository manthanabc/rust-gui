use pixel_sdl2::Draw;

fn main() {
	let mut draw = Draw::new(400, 400);
	draw.stroke(32);
	draw.rect(0, 0, 200, 200);

	'draw: loop {
		draw.line(0, 0, 400, 400);

		draw.handel_window_events();
	}
}