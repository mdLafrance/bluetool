pub fn get_icon_for_bt_type(bt_type: &str) -> &'static str {
    match bt_type {
        "audio-headphones" => " ",
        "audio-headset" => " ",
        "input-keyboard" => " ",
        "audio-card" => "󱀞 ",
        _ => " ",
    }
}
