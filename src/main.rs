use std::vec;

use speedy2d::{Window, window::{WindowHandler, WindowCreationOptions, WindowSize, MouseButton, WindowHelper}, color::Color, dimen::Vector2, Graphics2D, shape::Rectangle};

enum EEGameState {
    TitleScreen,
    MainMenu
}

#[derive(Clone)]
struct Button {
    rect: Rectangle<f32>,
    text: String,
    click_fn: fn(&mut WinHandler)
}

impl Button {
    fn new(rect: Rectangle<f32>, text: String, click_fn: fn(&mut WinHandler)) -> Button {
        Button { rect, text, click_fn }
    }
}

struct WinHandler {
    game_state: EEGameState,
    self_pos: Vector2<f32>,
    self_vel: Vector2<f32>,
    ui_buttons: Vec<Button>,
    mouse_pos: Vector2<f32>
}

impl WinHandler {
    fn button_click(&mut self, pos: Vector2<f32>, click_one: bool) {
        let btns: &Vec<Button> = &self.ui_buttons.clone();
        for b in btns {
            if b.rect.contains(pos) {
                (b.click_fn)(self);
                if click_one { break; }
            }
        }
    }
    fn switchtogame(&mut self) {
        self.game_state = EEGameState::MainMenu;
    }
    fn draw_buttons(&mut self, graphics: &mut Graphics2D) {
        for b in &self.ui_buttons {
            graphics.draw_rectangle(b.rect.clone(), Color::from_hex_argb(0x333380cc));
        }
    }
}

impl WindowHandler for WinHandler {
    fn on_draw(&mut self, _helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        match self.game_state {
            EEGameState::TitleScreen => {
                graphics.clear_screen(Color::from_hex_rgb(0x00ccff));
                graphics.draw_circle(Vector2::new(400.0, 225.0), 50.0, Color::from_hex_rgb(0xccff00));
            },
            EEGameState::MainMenu => {
                graphics.clear_screen(Color::from_hex_rgb(0xccff00));
                graphics.draw_circle(Vector2::new(400.0, 225.0), 50.0, Color::from_hex_rgb(0x00ccff));
            }
        }
        self.draw_buttons(graphics);
    }
    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        match button {
            MouseButton::Left => {},
            MouseButton::Middle => {},
            MouseButton::Right => {},
            MouseButton::Other(_) => {},
        }
    }
    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: Vector2<f32>) {
        self.mouse_pos = position;
    }
    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        match button {
            MouseButton::Left => { self.button_click(self.mouse_pos, true); },
            MouseButton::Middle => {},
            MouseButton::Right => {},
            MouseButton::Other(_) => {},
        }
    }
}

fn main() {
    Button::new(Rectangle::new(Vector2::new(100.0, 100.0), Vector2::new(100.0, 100.0)), String::from("bvrruh"), |wh: &mut WinHandler| { wh.game_state = EEGameState::MainMenu; });
    let window = Window::new_with_options("Everybody Edits: Oxidized",
        WindowCreationOptions::new_windowed(
            WindowSize::ScaledPixels(Vector2::new(1280.0, 720.0)),
            // Some(WindowPosition::Center),
            None,
        )
    ).unwrap();
    window.run_loop(WinHandler {
        game_state: EEGameState::TitleScreen,
        self_pos: Vector2 { x: 0.0, y: 0.0 },
        self_vel: Vector2 { x: 0.0, y: 0.0 },
        ui_buttons: vec![],
        mouse_pos: Vector2 { x: 0.0, y: 0.0 }
    });
}
