//#![allow(unused)]
use gwm::*;
use penrose::{
    builtin::layout::{
        transformers::{Gaps, ReserveTop},
        MainAndStack,
    },
    core::{bindings::parse_keybindings_with_xmodmap, Config, WindowManager},
    extensions::hooks::{
        add_named_scratchpads, manage::FloatingCentered, NamedScratchPad, SpawnOnStartup,
    },
    stack,
    x::query::ClassName,
    x11rb::RustConn,
    Result,
};
use std::collections::HashMap;

fn main() -> Result<()> {
    ////////////////////////////////
    //           HOOKS            //
    ///////////////////////////////
    //  Startup hook: run before entering the main event loop
    let startup_hook = SpawnOnStartup::boxed("/home/sergiu/scripts/alpha/gwm_start.sh");
   
    ////////////////////////////////
    //         SCRATCHPADS        //
    ///////////////////////////////
    let (spt, terminal) = NamedScratchPad::new(
        "terminal",
        "st -c StScratchpad",
        ClassName("StScratchpad"),
        FloatingCentered::new(0.8, 0.8),
        true,
    );
    let (spr, ranger) = NamedScratchPad::new(
        "ranger",
        "st -c Ranger -e ranger",
        ClassName("Ranger"),
        FloatingCentered::new(0.5, 0.5),
        true,
    );
    let scratchpads = vec![spt, spr];

    ////////////////////////////////
    //           LAYOUTS          //
    ///////////////////////////////
    let layouts = stack!(MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP))
        .map(|layout| ReserveTop::wrap(Gaps::wrap(layout, OUTER_PX, INNER_PX), BAR_HEIGHT_PX));

    ////////////////////////////
    //        BUILD WM        //
    ///////////////////////////
    let config = Config {
        default_layouts: layouts,
        //manage_hook: manage_hooks,
        startup_hook: Some(startup_hook),
        focused_border: BORDER_FOCUS.into(),
        normal_border: BORDER_NORMAL.into(),
        focus_follow_mouse: true,
        tags: ["1", "2", "3", "4", "5", "6"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        ..Config::default()
    };

    let conn = RustConn::new()?;
    let key_bindings =
        parse_keybindings_with_xmodmap(bindings::raw_key_bindings(terminal, ranger))?;

    let wm = WindowManager::new(config, key_bindings, HashMap::new(), conn)?;

    let bar = bar::bar().unwrap();

    // hook the scratchpads
    let wm = add_named_scratchpads(wm, scratchpads);

    // hook the bar
    let wm = bar.add_to(wm);

    wm.run()
}
