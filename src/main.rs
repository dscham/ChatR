use std::future::Future;
use std::net::TcpListener;

use console_engine::{
    ConsoleEngine,
    events::Event,
    forms::{Form, FormField, FormOptions, FormStyle, FormValue, Text},
    KeyCode, KeyModifiers, rect_style::BorderStyle,
};
use console_engine::crossterm::event::KeyEvent;

use crate::discover::Discovery;
use crate::Screen::PEERS;

mod peers;
mod chat;
mod settings;
mod discover;

fn main() {
    let mut engine = ConsoleEngine::init(80, 25, 10).unwrap();
    engine.set_title("ChatR");

    let theme = FormStyle {
        border: Some(BorderStyle::new_light()),
        ..Default::default()
    };

    let mut form = Form::new(
        20,
        6,
        FormOptions {
            style: theme,
            ..Default::default()
        },
    );

    form.build_field::<Text>(
        "username",
        FormOptions {
            style: theme,
            label: Some("Enter Username"),
            ..Default::default()
        },
    );

    form.set_active(true);

    while !form.is_finished() {
        // Poll next event
        match engine.poll() {
            // A frame has passed
            Event::Frame => {
                engine.clear_screen();
                engine.print_screen(5, 1, form.draw((engine.frame_count % 8 > 3) as usize));
                engine.draw();
            }

            // exit with Escape
            Event::Key(KeyEvent {
                           code: KeyCode::Esc,
                           modifiers: _,
                       }) => {
                break;
            }

            // exit with CTRL+C
            Event::Key(KeyEvent {
                           code: KeyCode::Char('c'),
                           modifiers: KeyModifiers::CONTROL,
                       }) => {
                break;
            }
            // Let the form handle the unhandled events
            event => form.handle_event(event),
        }
    }

    let mut username = String::new();
    if form.is_finished() {

        // Get the output of each fields
        if let Ok(FormValue::String(name)) = form.get_validated_field_output("username") {
            username = name;
        }
    }

    let mut discovery = Discovery::new(username.as_str());
    discovery.start();

    let mut current_screen = Screen::HOME;

    loop {
        engine.clear_screen();

        match current_screen {
            Screen::HOME => {
                engine.print(0, 0, format!("Welcome to ChatR, {}!", username).as_str());
                engine.print(0, 1, "(p)eers, (s)ettings, (q)uit");
            }
            Screen::PEERS => {
                engine.print(0, 0, "ChatR");
                engine.print(0, 1, "Peers");
                engine.print(0, 2, "(h)ome, (s)ettings, (q)uit");
                let mut i = 3;
                unsafe {
                    for peer in peers::PEERS.get_peers() {
                        engine.print(0, i -2, format!("{}) {}", i, peer.name).as_str());
                        i += 1;
                    }
                }
            }
            Screen::PEER => {
                let peer = unsafe { peers::PEERS.get_peers().get(0).unwrap() };
                engine.print(0, 0, "ChatR");
                engine.print(0, 1, peer.name.as_str());
                engine.print(0, 2, "(h)ome, (p)eers, (s)ettings, (q)uit");


            }
            Screen::SETTINGS => {
                engine.print(0, 0, "ChatR");
                engine.print(0, 1, "Settings");
                engine.print(0, 2, "(h)ome, (p)eers, (q)uit");
                engine.print(0, 3, format!("1) Toggle discovery - {:?}", !discovery.is_running()).as_str());
            }
        }

        match engine.poll() {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => {
                        discovery.stop();
                        break;
                    }
                    KeyCode::Char('h') => {
                        current_screen = Screen::HOME;
                    }
                    KeyCode::Char('p') => {
                        current_screen = Screen::PEERS;
                    }
                    KeyCode::Char('s') => {
                        current_screen = Screen::SETTINGS;
                    }
                    KeyCode::Char('1') => {
                        current_screen = Screen::PEER;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        engine.draw();
    }
}


enum Screen {
    HOME,
    PEERS,
    PEER,
    SETTINGS,
}