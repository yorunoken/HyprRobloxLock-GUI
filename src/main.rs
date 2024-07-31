use druid::widget::{Button, Container, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, Widget, WidgetExt, WindowDesc};

use locker::CursorLocker;
use std::sync::atomic::Ordering;

mod config;
mod dotool;
mod locker;

#[derive(Clone, Data, Lens)]
struct AppState {
    hyprland_status: bool,
    is_locked: bool,
    locker: CursorLocker,
    edge_offset: String,
}

impl Data for CursorLocker {
    fn same(&self, other: &Self) -> bool {
        self.is_on.load(Ordering::SeqCst) == other.is_on.load(Ordering::SeqCst)
    }
}

fn main_ui() -> impl Widget<AppState> {
    let title = Label::new("Roblox cursor locker for Waydroid").with_text_size(24.0);

    let status_label = Label::new(|data: &AppState, _: &Env| {
        match data.hyprland_status {
        true => format!("Running on Hyprland: yes"),
        false => format!("Running on Hyprland: no\nThis application can only run on Hyprland, so please open it from a Hyprland instance."),
    }
    }).with_text_alignment(druid::text::TextAlignment::Center);

    let keybind = Label::new("Keybind to quit: CTRL+SHIFT+X")
        .with_text_alignment(druid::text::TextAlignment::Center);

    let offset = Container::new(
        Flex::row()
            .with_child(Label::new("Edge Offset: "))
            .with_child(
                TextBox::new()
                    .with_placeholder("Value here")
                    .lens(AppState::edge_offset),
            ),
    );

    let toggle_button = Button::new(|data: &AppState, _: &Env| {
        if data.is_locked {
            "Toggle lock OFF".to_string()
        } else {
            "Toggle lock ON".to_string()
        }
    })
    .on_click(
        |_ctx, data: &mut AppState, _env| match data.edge_offset.parse::<i64>() {
            Ok(edge_offset) => {
                data.is_locked = !data.is_locked;
                data.locker.is_on.store(data.is_locked, Ordering::SeqCst);
                if data.is_locked {
                    data.locker.start(edge_offset);
                }
            }
            Err(_) => {
                panic!()
            }
        },
    )
    .disabled_if(|data, _env| !data.hyprland_status);

    let starting_label = Label::new(|data: &AppState, _: &Env| match data.is_locked {
        true => format!(
            "Starting in 5 seconds, please have your cursor in the monitor where Waydroid is.",
        ),
        false => format!("Waiting.."),
    })
    .with_text_color(druid::Color::RED)
    .with_text_alignment(druid::text::TextAlignment::Center);

    let force_kill =
        Button::new("Force kill").on_click(|_ctx, _data: &mut AppState, _env| panic!());

    Container::new(
        Flex::column()
            .with_child(title)
            .with_spacer(20.0)
            .with_child(status_label)
            .with_spacer(5.0)
            .with_child(keybind)
            .with_spacer(10.0)
            .with_child(offset)
            .with_spacer(10.0)
            .with_child(starting_label)
            .with_child(toggle_button)
            .with_spacer(5.0)
            .with_child(force_kill),
    )
    .padding(10.0)
    .center()
}

fn main() {
    let window = WindowDesc::new(main_ui())
        .window_size((700.0, 300.0))
        .title("HyprRobloxLock");

    let data = AppState {
        hyprland_status: test_hyprland(),
        is_locked: false,
        locker: CursorLocker::new(),
        edge_offset: "20".to_string(),
    };

    AppLauncher::with_window(window)
        .launch(data)
        .expect("Failed to launch application");
}

fn test_hyprland() -> bool {
    match std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE") {
        Some(env) => !env.is_empty(),
        None => false,
    }
}
