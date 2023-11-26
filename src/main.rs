use iced::widget::{button, container, row, text_input, Button, Column, Container, Text};
use iced::widget::{column, pick_list};
use iced::Alignment;
use iced::Renderer;
use iced::Sandbox;
use iced::Settings;
use rfd::FileDialog;
use std::borrow::Cow;
use std::path::PathBuf;
use std::process::Command;
impl Sandbox for VideoConverter {
    type Message = VideoMessage;

    fn new() -> Self {
        VideoConverter {
            from_file_path: String::from(""),
            from_format: String::from(""),
            to_format: VideoFormat::Mp4,
            to_file_path: String::from(""),
        }
    }

    fn title(&self) -> String {
        return String::from("Video Converter");
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            VideoMessage::ConvertFile => {
                let output = Command::new("ffmpeg")
                    .arg("-i")
                    .arg(self.from_file_path.as_str())
                    .arg("-c")
                    .arg("copy")
                    .arg(self.to_file_path.as_str())
                    .output();
            }
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
            VideoMessage::ToSelected(i) => {
                self.to_file_path = i;
            }
            VideoMessage::FormatSelected(i) => {
                self.to_format = i;
                println!("{:#?}", self.to_format);
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let file_picker = button("Open File").on_press(VideoMessage::OpenFile);
        let convert_to = text_input(
            "Enter the destination path of the video you want to convert to",
            self.to_file_path.as_str(),
        )
        .size(20)
        .on_input(VideoMessage::ToSelected);
        let to_format: iced::widget::PickList<'_, VideoFormat, VideoMessage, Renderer> = pick_list(
            &VideoFormat::ALL[..],
            Some(self.to_format),
            VideoMessage::FormatSelected,
        )
        .placeholder("Choose a video format...");
        let conver_button: iced::widget::Button<'_, VideoMessage, Renderer> =
            button("Convert!").on_press(VideoMessage::ConvertFile);
        column![
            "Welcome to the Video Converter!",
            file_picker,
            convert_to,
            conver_button
        ]
        .align_items(Alignment::Center)
        .into()
    }
}

struct VideoConverter {
    from_file_path: String,
    from_format: String,
    to_format: VideoFormat,
    to_file_path: String,
}

impl std::fmt::Display for VideoFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VideoFormat::Mp4 => "MP4",
                VideoFormat::Mkv => "MKV",
            }
        )
    }
}
fn load_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("video", &["mp4", "mkv"])
        .set_directory("~")
        .pick_file()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VideoFormat {
    Mp4,
    Mkv,
}

impl VideoFormat {
    const ALL: [VideoFormat; 2] = [VideoFormat::Mp4, VideoFormat::Mkv];
}

#[derive(Debug, Clone)]
enum VideoMessage {
    ConvertFile,
    ErrorConverting,
    OpenFile,
    FileSelected,
    FromSelected,
    ToSelected(String),
    FormatSelected(VideoFormat),
}

fn main() -> iced::Result {
    VideoConverter::run(Settings::default())
}
