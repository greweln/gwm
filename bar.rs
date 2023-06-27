// #![allow(unused)]
use crate::{BAR_BG, BAR_HEIGHT_PX, BAR_HIGHLIGHT, FONT, FONT_SIZE, WHITE};
use chrono::Local;
use penrose::{util::spawn_for_output_with_args, x::XConn};
use penrose_ui::{
    bar::{
        widgets::{ActiveWindowName, IntervalText, Workspaces},
        Position, StatusBar,
    },
    core::TextStyle,
};
use std::time::Duration;

pub fn bar<X: XConn>() -> penrose_ui::Result<StatusBar<X>> {
    let style = TextStyle {
        font: FONT.to_string(),
        point_size: FONT_SIZE,
        fg: WHITE.into(),
        bg: Some(BAR_BG.into()),
        padding: (2.0, 2.0),
    };

    let active_screen_style = TextStyle {
        bg: Some(BAR_HIGHLIGHT.into()),
        fg: BAR_HIGHLIGHT.into(),
        padding: (6.0, 4.0),
        ..style.clone()
    };

    StatusBar::try_new(
        Position::Top,
        BAR_HEIGHT_PX,
        style.bg.unwrap_or_else(|| 0x000000.into()),
        &[&style.font],
        vec![
            Box::new(Workspaces::new(&style, BAR_HIGHLIGHT, BAR_BG)),
            Box::new(ActiveWindowName::new(1, &active_screen_style, true, false)),
            Box::new(weather(&style)),
            Box::new(time(&style)),
            Box::new(audio(&style)),
        ],
    )
}

fn weather(style: &TextStyle) -> IntervalText {
    IntervalText::new(style, get_weather_text, Duration::from_secs(60 * 60))
}

fn get_weather_text() -> String {
    let val = spawn_for_output_with_args("curl", &["-s", "http://wttr.in?format=%f"])
        .unwrap_or_default()
        .trim()
        .to_string();
    format!("󰖨 {val} ")
}

fn get_time() -> String {
    let time = Local::now();
    format!(
        "  {}   {} ",
        time.format("%a-%d-%b"),
        time.format("%H:%M:%S")
    )
}
fn time(style: &TextStyle) -> IntervalText {
    IntervalText::new(style, get_time, Duration::from_secs(1))
}

fn audio(style: &TextStyle) -> IntervalText {
    IntervalText::new(style, get_audio, Duration::from_millis(200))
}

fn get_audio() -> String {
    let vol_out = spawn_for_output_with_args("pactl", &["get-sink-volume", "@DEFAULT_SINK@"])
        .unwrap()
        .trim()
        .to_string();
    let mute_out = spawn_for_output_with_args("pactl", &["get-sink-mute", "@DEFAULT_SINK@"])
        .unwrap()
        .trim()
        .to_string();

    let idx = vol_out.find('%').unwrap();
    let vol = vol_out.get(idx - 3..idx).unwrap().trim_start();

    let icon = if mute_out.contains("yes") {
        "婢"
    } else {
        match vol.parse::<u8>().unwrap() {
            0 => "婢",
            _n @ 1..=100 => "奔",
            _ => "ﱛ",
        }
    };

    format!(" {} {} ", icon, vol)
}
