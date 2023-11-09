use std::path::PathBuf;

use iced::widget::column;
use iced::widget::{button, container, row, Button, Column, Container, Text};
use iced::Sandbox;
use iced::Settings;
use rfd::FileDialog;
impl Sandbox for VideoConverter {
    type Message = VideoMessage;

    fn new() -> Self {
        VideoConverter {
            from_file_path: String::from(""),
            from_format: String::from(""),
            to_format: String::from(""),
            to_file_path: String::from(""),
        }
    }

    fn title(&self) -> String {
        return String::from("Video Converter");
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            VideoMessage::ConvertFile => todo!(),
            VideoMessage::ErrorConverting => todo!(),
            VideoMessage::OpenFile => {
                let path = load_file()
                    .unwrap_or("".into())
                    .into_os_string()
                    .into_string()
                    .unwrap();
                if path == "" {
                    panic!("FileNotIncluded >:*")
                }
                self.from_file_path = path;
                println!("{:#?}", self.from_file_path)
            }
            VideoMessage::FileSelected => todo!(),
            VideoMessage::FromSelected => todo!(),
            VideoMessage::ToSelected => todo!(),
            VideoMessage::DestinationSelected => todo!(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let file_picker = button("Open File").on_press(VideoMessage::OpenFile);
        column![file_picker].into()
    }
}

struct VideoConverter {
    from_file_path: String,
    from_format: String,
    to_format: String,
    to_file_path: String,
}

fn load_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("video", &["mp4", "mkv"])
        .set_directory("~")
        .pick_file()
}

#[derive(Debug, Clone)]
enum VideoMessage {
    ConvertFile,
    ErrorConverting,
    OpenFile,
    FileSelected,
    FromSelected,
    ToSelected,
    DestinationSelected,
}

fn main() -> iced::Result {
    VideoConverter::run(Settings::default())
}
