pub fn get_icon_for_bt_type(bt_type: &str) -> &'static str {
    match bt_type {
        "audio-headphones" | "headphones" => " ",
        "audio-headset" | "headset" => " ",
        "speaker" | "audio-speakers" | "car-audio" | "audio" => "󰓃 ",
        "input-keyboard" | "keyboard" => " ",
        "audio-card" => "󱀞 ",
        "computer" | "laptop" | "tablet" => " ",
        "phone" => " ",
        "input-mouse" | "mouse" => "󰍽 ",
        "input-gaming" | "gamepad" | "controller" => "󰊗 ",
        "printer" => "󰐪 ",
        "smart-light" | "lightbulb" => "󰌵 ",
        _ => " ",
    }
}
