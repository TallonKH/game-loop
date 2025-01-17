use game_loop::game_loop;

// For convenience, game_loop re-exports tao so you don't need to add it as
// an additional dependency of your crate.

use game_loop::tao::event::{Event, WindowEvent};
use game_loop::tao::event_loop::EventLoop;
use game_loop::tao::menu::{MenuBar, MenuItem};
use game_loop::tao::window::{Window, WindowBuilder};

fn main() {
    let mut file_menu = MenuBar::new();
    file_menu.add_native_item(MenuItem::Quit);

    let mut menu = MenuBar::new();
    menu.add_submenu("File", true, file_menu);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_menu(menu).build(&event_loop).unwrap();

    let game = Game::new();

    game_loop(event_loop, window, game, 240, 0.1, |g| {
        g.game.your_update_function();
    }, |g| {
        g.game.your_render_function(&g.window);
    }, |g, event| {
        if !g.game.your_window_handler(event) { g.exit(); }
    });
}

#[derive(Default)]
struct Game {
    counter: u32,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn your_update_function(&mut self) {
        self.counter += 1;
    }

    pub fn your_render_function(&self, window: &Window) {
        window.set_title(&format!("Counter: {}", self.counter));
    }

    // A very simple handler that returns false when CloseRequested is detected.
    pub fn your_window_handler(&self, event: &Event<()>) -> bool {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    return false;
                }
                _ => {}
            },
            _ => {}
        }

        true
    }
}
