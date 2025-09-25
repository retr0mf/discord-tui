use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Layout, Size},
    style::{Color, Modifier, Style, Styled, Stylize},
    text::Line,
    widgets::{Block, BorderType, Padding, Paragraph, RatatuiLogo, RatatuiLogoSize},
};
use std::{collections::HashMap, vec};

// └

const VERSION: &str = "0.01a";

#[derive(Default, Clone)]
struct User {
    name: String,
    username: String,
    is_friend: bool,
    account_creation_time: (),
}

impl User {
    fn new(name: String, username: String, is_friend: bool, account_creation_time: ()) -> User {
        User {
            name,
            username,
            is_friend,
            account_creation_time,
        }
    }
}

#[derive(Default, Clone)]
struct Message {
    sender: User,
    contents: String,
    sent_at: (), // Timestamp planned
}

#[derive(Default, Clone)]
struct Chat {
    user: User,
    messages: Vec<Message>,
}

#[derive(Default, Clone)]
struct Status {
    description: String,
    is_good: bool,
}

#[derive(Default, Clone)]
pub struct App {
    logged_user: User,
    chats: Vec<Chat>,
    friends: Vec<User>,
    is_running: bool,
    status: Status,
}

impl App {
    pub fn run(&mut self, mut term: DefaultTerminal) -> () {
        self.is_running = true;

        self.status = Status {
            description: String::from("OK"),
            is_good: true,
        };

        while self.is_running {
            term.draw(|frame| self.render(frame)).unwrap();
            self.handle_kbd_events();
        }
    }

    pub fn render(&mut self, frame: &mut Frame) -> () {
        let title_st = Style::new().bold().italic();
        let border_type = BorderType::Rounded;

        let outer = Block::bordered()
            .title(Line::from(format!("Discord-TUI ver. {}", VERSION)).left_aligned())
            .title_bottom(Line::from(format!("Status: {}", self.status.description)).left_aligned())
            .title_bottom(Line::from("Show Servers(Ctrl-S)  Show Users(Ctrl-U)").centered())
            .title_bottom(Line::from("Leader HK: Ctrl-L").right_aligned())
            .title_style(title_st)
            .border_type(border_type);

        let app_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(outer.inner(frame.area()));

        // let inner = Block::bordered().title(Line::from("Inner block").centered());
        let users_block = Block::bordered()
            .title(Line::from("Users").centered())
            .border_type(border_type);
        let active_chat_block = Block::bordered()
            .title(Line::from("Chat with @retr0").centered())
            .border_type(border_type);

        frame.render_widget(outer, frame.area());
        frame.render_widget(users_block, app_layout[0]);
        // frame.render_widget(active_chat_block, app_layout[1]);
        frame.render_widget(active_chat_block, app_layout[1]);
    }

    // fn compose_user_block(&mut self, ) -> Block {
    // }

    fn fetch_server_info(&mut self) -> Result<(), String> {
        self.logged_user = User::new("retr00".to_string(), "retr0912".to_string(), false, ());
        self.chats = vec![Chat {
            user: User {
                name: "Test Test".to_string(),
                username: "testacc".to_string(),
                is_friend: true,
                account_creation_time: (),
            },
            messages: vec![
                Message {
                    sender: self.logged_user.clone(),
                    contents: "Hello, Test!".to_string(),
                    sent_at: (),
                },
                Message {
                    sender: User {
                        name: "Test Test".to_string(),
                        username: "testacc".to_string(),
                        is_friend: true,
                        account_creation_time: (),
                    },
                    contents: "Hello, Test!".to_string(),
                    sent_at: (),
                },
            ],
        }];

        self.friends = vec![User {
            name: "Test Test".to_string(),
            username: "testacc".to_string(),
            is_friend: true,
            account_creation_time: (),
        }];
        Ok(())
    }

    fn handle_kbd_events(&mut self) -> () {
        match event::read().unwrap() {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.keypressed_event(key),
            _ => {}
        }
    }

    fn keypressed_event(&mut self, key: KeyEvent) -> () {
        match (key.modifiers, key.code) {
            (_, KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('C') | KeyCode::Char('c')) => self.quit(),
            _ => {}
        }
    }

    fn quit(&mut self) -> () {
        self.is_running = false;
    }
}
