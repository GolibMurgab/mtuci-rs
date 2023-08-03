use druid::{AppLauncher, WindowDesc, Color, theme::WINDOW_BACKGROUND_COLOR};
mod data;
use data::{Todo, Delegate};
mod ui;
use ui::build_ui;

mod controllers;

pub fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("Ту Ду")
        .window_size((500.0, 800.0));

    let initial_data = Todo::load_from_json();

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .configure_env(|env, _state| {
            env.set(WINDOW_BACKGROUND_COLOR, Color::from_hex_str("FFCC00")
                .expect("Неверная кодировка"));
        })
        .launch(initial_data)
        .expect("Ошибка при запуске приложения");
}