use std::{
    sync::{Arc, Mutex},
    thread,
};

use gameman::{
    consts::display::{DISPLAY_SIZE_X, DISPLAY_SIZE_Y},
    gpu::states::Color,
    GameBoy,
};
use macroquad::{
    color::{Color as MacroColor, WHITE},
    math::vec2,
    miniquad::FilterMode,
    texture::{draw_texture_ex, DrawTextureParams, Image, Texture2D},
    window::{clear_background, next_frame},
};

#[macroquad::main("Main")]
async fn main() {
    // ROM Loading
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("You need to specify the rom file");
        std::process::exit(1);
    }

    let rom_path = args.last().unwrap();

    // We need to make the emulator run on a separate thread, because macroquad's
    // `next_frame()` uses vsync, so the emulator will run based on your monitor refresh
    // rate, and it will be insanelyyy slow, my 60hz monitor took 10 minutes to render
    // ~10,000 CPU instructions, this is so fast that the same runs in <0.2s.
    // Two `Arc`s because one is for the "running" thread and one for the "rendering"
    // thread
    let gameboy = Arc::new(Mutex::new(GameBoy::new(&rom_path).unwrap()));
    let gameboy_clone = gameboy.clone();

    let mut image = Image::gen_image_color(DISPLAY_SIZE_X as u16, DISPLAY_SIZE_Y as u16, WHITE);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest); // without this it will be blurry

    // "Running" thread
    thread::spawn(move || loop {
        gameboy.lock().unwrap().step();
    });

    // "Rendering" thread
    loop {
        clear_background(MacroColor::from_rgba(0, 255, 0, 255));

        for (y_coordinate, y) in gameboy_clone.lock().unwrap().gpu.screen.iter().enumerate() {
            for (x_coordinate, x) in y.iter().enumerate() {
                image.set_pixel(
                    x_coordinate as u32,
                    y_coordinate as u32,
                    match x {
                        Color::Dark => MacroColor::from_rgba(0, 0, 0, 255),
                        Color::MediumlyDark => MacroColor::from_rgba(55, 55, 55, 255),
                        Color::MediumlyLight => MacroColor::from_rgba(155, 155, 155, 255),
                        Color::Light => MacroColor::from_rgba(255, 255, 255, 255),
                    },
                );
            }
        }

        texture.update(&image);
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(400., 400.)),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
