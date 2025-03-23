#[derive(Debug)]
pub struct Colors {
    pub default: u32,
    pub primary: u32,
    pub secondary: u32,
    pub success: u32,
    pub danger: u32,
    pub warning: u32,
    pub azoxo: u32,
    pub green: u32,
    pub yellow: u32,
    pub fuchsia: u32,
    pub magic: u32,
    pub developer: u32,
    pub balance: u32,
    pub brilliance: u32,
    pub nitro: u32,
    pub bravery: u32,
}

#[derive(Debug)]
pub struct Emojis {
    pub static_emojis: StaticEmojis,
}

#[derive(Debug)]
pub struct StaticEmojis {
    pub back: &'static str,
    pub check: &'static str,
    pub close: &'static str,
    pub home: &'static str,
    pub list: &'static str,
    pub lock: &'static str,
    pub next: &'static str,
    pub pause: &'static str,
    pub resume: &'static str,
    pub stop: &'static str,
    pub view: &'static str,
}

pub const COLORS: Colors = Colors {
    default: 0x2B2D31,
    primary: 0x3b82f6,
    secondary: 0x4f545c,
    success: 0x22c55e,
    danger: 0xED4245,
    warning: 0xfbbd23,
    azoxo: 0x5865F2,
    green: 0x57F287,
    yellow: 0xFEE75C,
    fuchsia: 0xEB459E,
    magic: 0xc026d3,
    developer: 0x3e70dd,
    balance: 0x45ddc0,
    brilliance: 0xf07d5f,
    nitro: 0xff6bfa,
    bravery: 0x9c84ef,
};

pub const EMOJIS: Emojis = Emojis {
    static_emojis: StaticEmojis {
        back: "1269441014251388959",
        check: "1269441016063070288",
        close: "1269441017573281914",
        home: "1269441019670429716",
        list: "1269441020936851518",
        lock: "1269441022467899433",
        next: "1269441023990567053",
        pause: "1269441025198264391",
        resume: "126944102664123195",
        stop: "1269441100884607077",
        view: "1269441030391070750",
    },
};
