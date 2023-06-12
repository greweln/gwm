use crate::{apps::*, KeyHandler};
use penrose::{
    builtin::{
        actions::{exit, modify_with, send_layout_message, spawn},
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    extensions::hooks::ToggleNamedScratchPad,
    map,
};
use std::collections::HashMap;

pub fn raw_key_bindings(
    terminal: ToggleNamedScratchPad,
    gpass: ToggleNamedScratchPad,
    // exit_menu: ToggleNamedScratchPad,
) -> HashMap<String, KeyHandler> {
    let mut raw_bindings = map! {
         map_keys: |k: &str| k.to_string();

    // Windows
         "A-space" => modify_with(|cs| cs.focus_down()),
         "A-S-j" => modify_with(|cs| cs.swap_down()),
         "A-S-k" => modify_with(|cs| cs.swap_up()),
         "A-q" => modify_with(|cs| cs.kill_focused()),
         // "A-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused),

         // Workspaces
         "A-Tab" => modify_with(|cs| cs.toggle_tag()),
         "A-h" => modify_with(|cs| cs.next_screen()),
         "A-l" => modify_with(|cs| cs.previous_screen()),
         "A-S-h" => modify_with(|cs| cs.drag_workspace_forward()),
         "A-S-l" => modify_with(|cs| cs.drag_workspace_backward()),

         // Layouts
         "A-Up" => send_layout_message(|| IncMain(1)),
         "A-Down" => send_layout_message(|| IncMain(-1)),
         "A-Right" => send_layout_message(|| ExpandMain),
         "A-Left" => send_layout_message(|| ShrinkMain),

         // Launchers
         "A-m" => spawn("maim -s /home/sergiu/Downloads/screenshot.png"),
         "A-r" => menu(),
         "M-r" => spawn("dmenu_run"),
         "A-Return" => spawn("st"),

         "A-g" => Box::new(terminal),
         "A-p" => Box::new(gpass),
         // "A-x" => Box::new(exit_menu),

         // System control
         "XF86AudioLowerVolume" => spawn("pactl set-sink-volume @DEFAULT_SINK@ -5%"),
         "XF86AudioRaiseVolume" => spawn("pactl set-sink-volume @DEFAULT_SINK@ +5%"),
         "XF86AudioMute" => spawn("pactl set-sink-mute @DEFAULT_SINK@ toggle"),
         "XF86AudioMicMute" => spawn("pactl set-source-mute @DEFAULT_SOURCE@ toggle"),
         "XF86MonBrightnessUp" => spawn("/home/sergiu/scripts/alpha/brightness_screen.sh +"),
         "XF86MonBrightnessDown" => spawn("/home/sergiu/scripts/alpha/brightness_screen.sh -"),
         "M-l" => spawn("slock"),



         // Session management
         "A-C-x" => exit(),


             };
    // let map = HashMap::from([
    //     ("1", ""),
    //     ("2", ""),
    //     ("3", ""),
    //     ("4", ""),
    //     ("5", ""),
    //     ("6", ""),
    // ]);

    for tag in &["1", "2", "3", "4", "5", "6"] {
        raw_bindings.extend([
            (
                format!("A-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("A-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}
