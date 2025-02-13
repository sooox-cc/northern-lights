mod theme;

use iced::{
    theme::Container as ThemeContainer,
    widget::{self, Container, Row, Text as WidgetText},
    Application, Background, Command, Element, Length, Settings,
};

use crate::theme::NordTheme; // import theme

// define application state
pub struct NothernLights {
    content: String,
}

// define message types
#[derive(Debug, Clone)]
pub enum Message {
    // TODO
}

// implement application trait for our app
impl Application for NothernLights {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            NothernLights {
                content: String::from("Welcome to Northern Lights"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Northern Lights")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // define custom styles (as fucntions)
        fn main_style(_theme: &iced::Theme) -> widget::container::Appearance {
            widget::container::Appearance {
                background: Some(Background::Color(NordTheme::NORD0)),
                text_color: Some(iced::Color::WHITE),
                ..Default::default()
            }
        }

        fn sidebar_style(_theme: &iced::Theme) -> widget::container::Appearance {
            widget::container::Appearance {
                background: Some(Background::Color(NordTheme::NORD1)),
                ..Default::default()
            }
        }

        // create sidebar component
        let sidebar = Container::new(WidgetText::new("Sidebar").size(16))
            .width(Length::Fixed(200.0))
            .height(Length::Fill)
            .style(ThemeContainer::Custom(Box::new(sidebar_style)));

        // create main content area
        let main_content = Container::new(WidgetText::new(&self.content).size(24))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(ThemeContainer::Custom(Box::new(main_style)));

        // combine into layout
        Row::new().push(sidebar).push(main_content).into()
    }
}

// main function
fn main() -> iced::Result {
    NothernLights::run(Settings::default())
}
