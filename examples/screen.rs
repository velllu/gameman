use std::{
    sync::{Arc, Mutex},
    thread,
};

use gameman::{
    consts::display::{DISPLAY_SIZE_X, DISPLAY_SIZE_Y},
    gpu::Color,
    GameBoy,
};
use macroquad::{
    color::{Color as MacroColor, WHITE},
    input::{is_key_down, KeyCode},
    math::vec2,
    miniquad::FilterMode,
    shapes::draw_line,
    texture::{draw_texture_ex, DrawTextureParams, Image, Texture2D},
    window::{clear_background, next_frame, screen_height, screen_width},
};

#[macroquad::main("Gameman Screen Debugger")]
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

    let scale = 4.;
    let width = scale * DISPLAY_SIZE_X as f32;
    let height = scale * DISPLAY_SIZE_Y as f32;

    // "Rendering" thread
    loop {
        let gb_x = screen_width() / 2. - width / 2.;
        let gb_y = screen_height() / 2. - height / 2.;

        clear_background(MacroColor::from_rgba(0x28, 0x28, 0x28, 255));

        for (y_coordinate, y) in gameboy_clone.lock().unwrap().gpu.screen.iter().enumerate() {
            for (x_coordinate, x) in y.iter().enumerate() {
                image.set_pixel(
                    x_coordinate as u32,
                    y_coordinate as u32,
                    match x {
                        Color::Dark => MacroColor::from_rgba(0x14, 0x2C, 0x38, 255),
                        Color::MediumlyDark => MacroColor::from_rgba(0x54, 0x8C, 0x70, 255),
                        Color::MediumlyLight => MacroColor::from_rgba(0xAC, 0xD4, 0x90, 255),
                        Color::Light => MacroColor::from_rgba(0xE8, 0xFC, 0xCC, 255),
                    },
                );
            }
        }

        texture.update(&image);
        draw_texture_ex(
            &texture,
            gb_x,
            gb_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                ..Default::default()
            },
        );

        if is_key_down(KeyCode::Space) {
            for i in 0..(DISPLAY_SIZE_Y / 8 + 1) {
                draw_line(
                    gb_x,
                    gb_y + (i as f32) * 8. * scale,
                    gb_x + width,
                    gb_y + (i as f32) * 8. * scale,
                    1.,
                    MacroColor::from_rgba(0xFF, 0, 0, 255),
                );
            }

            for i in 0..(DISPLAY_SIZE_X / 8 + 1) {
                draw_line(
                    gb_x + (i as f32) * 8. * scale,
                    gb_y,
                    gb_x + (i as f32) * 8. * scale,
                    gb_y + height,
                    1.,
                    MacroColor::from_rgba(0xFF, 0, 0, 255),
                );
            }
        }

        next_frame().await;
    }
}
