use monitor::{InputEvent, KeyValue};

// $ awk '{ print "    InputEvent::Key { down: false, key: KeyValue::Qcode(" $3 ") },"; }' qcode.rs

pub const EVENTS: &'static [InputEvent] = &[
    InputEvent::Key { down: false, key: KeyValue::Qcode("shift",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("shift_r",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("alt",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("alt_r",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("ctrl",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("ctrl_r",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("menu",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("esc",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("0",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("1",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("2",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("3",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("4",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("5",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("6",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("7",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("8",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("9",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("minus",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("equal",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("backspace",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("tab",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("a",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("b",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("c",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("d",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("e",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("f",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("g",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("h",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("i",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("j",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("k",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("l",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("m",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("n",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("o",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("p",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("q",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("r",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("s",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("t",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("u",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("v",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("w",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("x",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("y",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("z",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("ret",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("semicolon",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("apostrophe",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("grave_accent",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("backslash",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("comma",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("dot",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("slash",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("spc",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("caps_lock",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("f1",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("f2",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("f3",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("f4",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("f5",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("f6",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("f7",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("f8",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("f9",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("f10",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("f11",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("f12",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("num_lock",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("scroll_lock",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_divide",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_multiply",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_subtract",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_add",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_enter",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_decimal",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("sysrq",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_0",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_1",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_2",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_3",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_4",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_5",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_6",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_7",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_8",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_9",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("less",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("home",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("pgup",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("pgdn",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("end",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("left",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("up",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("down",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("right",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("insert",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("delete",) },
//    InputEvent::Key { down: false, key: KeyValue::Qcode(return) },
//    InputEvent::Key { down: false, key: KeyValue::Qcode(return) },
//    InputEvent::Key { down: false, key: KeyValue::Qcode(return) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("pause",) },
//    InputEvent::Key { down: false, key: KeyValue::Qcode() },
    InputEvent::Key { down: false, key: KeyValue::Qcode("kp_equals",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("meta_l",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("meta_r",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("power",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("bracket_left",) },
    InputEvent::Key { down: false, key: KeyValue::Qcode("bracket_right",) },
//    InputEvent::Key { down: false, key: KeyValue::Qcode(return) },
//    InputEvent::Key { down: false, key: KeyValue::Qcode(return) },
//    InputEvent::Key { down: false, key: KeyValue::Qcode(return) },
//    InputEvent::Key { down: false, key: KeyValue::Qcode(return) },
];
