use std::ops::BitAnd;

pub const MAX_SPLITSCREEN_CLIENT_BITS: u32 = 2;
pub const MAX_SPLITSCREEN_CLIENTS: u32 = 4;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Button(i32);

impl Button {
    pub const BUTTON_CODE_INVALID: Self = Self::new(-1);
    pub const BUTTON_CODE_NONE: Self = Self::new(0);
    pub const KEY_FIRST: Self = Self::new(0);
    pub const KEY_NONE: Self = Self::new(0);
    pub const KEY_0: Self = Self::new(1);
    pub const KEY_1: Self = Self::new(2);
    pub const KEY_2: Self = Self::new(3);
    pub const KEY_3: Self = Self::new(4);
    pub const KEY_4: Self = Self::new(5);
    pub const KEY_5: Self = Self::new(6);
    pub const KEY_6: Self = Self::new(7);
    pub const KEY_7: Self = Self::new(8);
    pub const KEY_8: Self = Self::new(9);
    pub const KEY_9: Self = Self::new(10);
    pub const KEY_A: Self = Self::new(11);
    pub const KEY_B: Self = Self::new(12);
    pub const KEY_C: Self = Self::new(13);
    pub const KEY_D: Self = Self::new(14);
    pub const KEY_E: Self = Self::new(15);
    pub const KEY_F: Self = Self::new(16);
    pub const KEY_G: Self = Self::new(17);
    pub const KEY_H: Self = Self::new(18);
    pub const KEY_I: Self = Self::new(19);
    pub const KEY_J: Self = Self::new(20);
    pub const KEY_K: Self = Self::new(21);
    pub const KEY_L: Self = Self::new(22);
    pub const KEY_M: Self = Self::new(23);
    pub const KEY_N: Self = Self::new(24);
    pub const KEY_O: Self = Self::new(25);
    pub const KEY_P: Self = Self::new(26);
    pub const KEY_Q: Self = Self::new(27);
    pub const KEY_R: Self = Self::new(28);
    pub const KEY_S: Self = Self::new(29);
    pub const KEY_T: Self = Self::new(30);
    pub const KEY_U: Self = Self::new(31);
    pub const KEY_V: Self = Self::new(32);
    pub const KEY_W: Self = Self::new(33);
    pub const KEY_X: Self = Self::new(34);
    pub const KEY_Y: Self = Self::new(35);
    pub const KEY_Z: Self = Self::new(36);
    pub const KEY_PAD_0: Self = Self::new(37);
    pub const KEY_PAD_1: Self = Self::new(38);
    pub const KEY_PAD_2: Self = Self::new(39);
    pub const KEY_PAD_3: Self = Self::new(40);
    pub const KEY_PAD_4: Self = Self::new(41);
    pub const KEY_PAD_5: Self = Self::new(42);
    pub const KEY_PAD_6: Self = Self::new(43);
    pub const KEY_PAD_7: Self = Self::new(44);
    pub const KEY_PAD_8: Self = Self::new(45);
    pub const KEY_PAD_9: Self = Self::new(46);
    pub const KEY_PAD_DIVIDE: Self = Self::new(47);
    pub const KEY_PAD_MULTIPLY: Self = Self::new(48);
    pub const KEY_PAD_MINUS: Self = Self::new(49);
    pub const KEY_PAD_PLUS: Self = Self::new(50);
    pub const KEY_PAD_ENTER: Self = Self::new(51);
    pub const KEY_PAD_DECIMAL: Self = Self::new(52);
    pub const KEY_LBRACKET: Self = Self::new(53);
    pub const KEY_RBRACKET: Self = Self::new(54);
    pub const KEY_SEMICOLON: Self = Self::new(55);
    pub const KEY_APOSTROPHE: Self = Self::new(56);
    pub const KEY_BACKQUOTE: Self = Self::new(57);
    pub const KEY_COMMA: Self = Self::new(58);
    pub const KEY_PERIOD: Self = Self::new(59);
    pub const KEY_SLASH: Self = Self::new(60);
    pub const KEY_BACKSLASH: Self = Self::new(61);
    pub const KEY_MINUS: Self = Self::new(62);
    pub const KEY_EQUAL: Self = Self::new(63);
    pub const KEY_ENTER: Self = Self::new(64);
    pub const KEY_SPACE: Self = Self::new(65);
    pub const KEY_BACKSPACE: Self = Self::new(66);
    pub const KEY_TAB: Self = Self::new(67);
    pub const KEY_CAPSLOCK: Self = Self::new(68);
    pub const KEY_NUMLOCK: Self = Self::new(69);
    pub const KEY_ESCAPE: Self = Self::new(70);
    pub const KEY_SCROLLLOCK: Self = Self::new(71);
    pub const KEY_INSERT: Self = Self::new(72);
    pub const KEY_DELETE: Self = Self::new(73);
    pub const KEY_HOME: Self = Self::new(74);
    pub const KEY_END: Self = Self::new(75);
    pub const KEY_PAGEUP: Self = Self::new(76);
    pub const KEY_PAGEDOWN: Self = Self::new(77);
    pub const KEY_BREAK: Self = Self::new(78);
    pub const KEY_LSHIFT: Self = Self::new(79);
    pub const KEY_RSHIFT: Self = Self::new(80);
    pub const KEY_LALT: Self = Self::new(81);
    pub const KEY_RALT: Self = Self::new(82);
    pub const KEY_LCONTROL: Self = Self::new(83);
    pub const KEY_RCONTROL: Self = Self::new(84);
    pub const KEY_LWIN: Self = Self::new(85);
    pub const KEY_RWIN: Self = Self::new(86);
    pub const KEY_APP: Self = Self::new(87);
    pub const KEY_UP: Self = Self::new(88);
    pub const KEY_LEFT: Self = Self::new(89);
    pub const KEY_DOWN: Self = Self::new(90);
    pub const KEY_RIGHT: Self = Self::new(91);
    pub const KEY_F1: Self = Self::new(92);
    pub const KEY_F2: Self = Self::new(93);
    pub const KEY_F3: Self = Self::new(94);
    pub const KEY_F4: Self = Self::new(95);
    pub const KEY_F5: Self = Self::new(96);
    pub const KEY_F6: Self = Self::new(97);
    pub const KEY_F7: Self = Self::new(98);
    pub const KEY_F8: Self = Self::new(99);
    pub const KEY_F9: Self = Self::new(100);
    pub const KEY_F10: Self = Self::new(101);
    pub const KEY_F11: Self = Self::new(102);
    pub const KEY_F12: Self = Self::new(103);
    pub const KEY_CAPSLOCKTOGGLE: Self = Self::new(104);
    pub const KEY_NUMLOCKTOGGLE: Self = Self::new(105);
    pub const KEY_SCROLLLOCKTOGGLE: Self = Self::new(106);
    pub const KEY_LAST: Self = Self::new(106);
    pub const KEY_COUNT: Self = Self::new(107);
    pub const MOUSE_FIRST: Self = Self::new(107);
    pub const MOUSE_LEFT: Self = Self::new(107);
    pub const MOUSE_RIGHT: Self = Self::new(108);
    pub const MOUSE_MIDDLE: Self = Self::new(109);
    pub const MOUSE_4: Self = Self::new(110);
    pub const MOUSE_5: Self = Self::new(111);
    pub const MOUSE_WHEEL_UP: Self = Self::new(112);
    pub const MOUSE_WHEEL_DOWN: Self = Self::new(113);
    pub const MOUSE_LAST: Self = Self::new(113);
    pub const MOUSE_COUNT: Self = Self::new(7);
    pub const JOYSTICK_FIRST: Self = Self::new(114);
    pub const JOYSTICK_FIRST_BUTTON: Self = Self::new(114);
    pub const JOYSTICK_LAST_BUTTON: Self = Self::new(241);
    pub const JOYSTICK_FIRST_POV_BUTTON: Self = Self::new(242);
    pub const JOYSTICK_LAST_POV_BUTTON: Self = Self::new(257);
    pub const JOYSTICK_FIRST_AXIS_BUTTON: Self = Self::new(258);
    pub const JOYSTICK_LAST_AXIS_BUTTON: Self = Self::new(305);
    pub const JOYSTICK_LAST: Self = Self::new(305);
    pub const BUTTON_CODE_LAST: Self = Self::new(306);
    pub const BUTTON_CODE_COUNT: Self = Self::new(307);
    pub const KEY_XBUTTON_UP: Self = Self::new(242);
    pub const KEY_XBUTTON_RIGHT: Self = Self::new(243);
    pub const KEY_XBUTTON_DOWN: Self = Self::new(244);
    pub const KEY_XBUTTON_LEFT: Self = Self::new(245);
    pub const KEY_XBUTTON_A: Self = Self::new(114);
    pub const KEY_XBUTTON_B: Self = Self::new(115);
    pub const KEY_XBUTTON_X: Self = Self::new(116);
    pub const KEY_XBUTTON_Y: Self = Self::new(117);
    pub const KEY_XBUTTON_LEFT_SHOULDER: Self = Self::new(118);
    pub const KEY_XBUTTON_RIGHT_SHOULDER: Self = Self::new(119);
    pub const KEY_XBUTTON_BACK: Self = Self::new(120);
    pub const KEY_XBUTTON_START: Self = Self::new(121);
    pub const KEY_XBUTTON_STICK1: Self = Self::new(122);
    pub const KEY_XBUTTON_STICK2: Self = Self::new(123);
    pub const KEY_XBUTTON_INACTIVE_START: Self = Self::new(124);
    pub const KEY_XSTICK1_RIGHT: Self = Self::new(258);
    pub const KEY_XSTICK1_LEFT: Self = Self::new(259);
    pub const KEY_XSTICK1_DOWN: Self = Self::new(260);
    pub const KEY_XSTICK1_UP: Self = Self::new(261);
    pub const KEY_XBUTTON_LTRIGGER: Self = Self::new(262);
    pub const KEY_XBUTTON_RTRIGGER: Self = Self::new(263);
    pub const KEY_XSTICK2_RIGHT: Self = Self::new(264);
    pub const KEY_XSTICK2_LEFT: Self = Self::new(265);
    pub const KEY_XSTICK2_DOWN: Self = Self::new(266);
    pub const KEY_XSTICK2_UP: Self = Self::new(267);

    const fn new(state: i32) -> Self {
        Self(state)
    }
}
