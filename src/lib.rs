use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;

fn main() {

	let sdl_context = sdl2::init().unwrap();
	let event_pump = sdl_context.event_pump().unwrap();
	let mouse_state = event_pump.mouse_state();
	let video_subsystem = sdl_context.video();
	
	// let window = WindowBuilder::new("Example", 800, 600).build().unwrap();
	let window = video_subsystem.unwrap().window("Example", 800, 600).build().unwrap();
	
	// Let's create a Canvas which we will use to draw in our Window
	let canvas : Canvas<Window> = window.into_canvas()
	    .present_vsync() //< this means the screen cannot
	    // render faster than your display rate (usually 60Hz or 144Hz)
	    .build().unwrap();

	loop {
		println!("{:?}",mouse_state.x());
	}

	for event in event_pump.poll_iter() {
	    
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();

		canvas.set_draw_color(Color::RGB(255, 210, 0));
		canvas.fill_rect(Rect::new(10, 10, 780, 580)).unwrap();

		canvas.set_draw_color(Color::RGB(0, 0, 0));

		for x in 0..800 {
		canvas.draw_point(Point::new(x, 300)).unwrap();		
		}

		canvas.present();
	}
}