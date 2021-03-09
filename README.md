# rust-gui

## a user friendly multy platform rust gui library 
  it uses sdl 2 and lets you create canvas and basic shapes like rectrangle in a easy to se way
  
  ``` rust
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
  ```
  
  ## getting started
  
   just copy past the lib file in your project src directory
    
   and add the following dependecy
    
    ```
    sdl2 = "0.30.0"
    ```
    
   and thas it .
    
