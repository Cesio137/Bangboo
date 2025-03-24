#[derive(Clone, Copy)]
#[repr(u32)]
pub enum EColor {
    Default = 0x2B2D31,
    Primary = 0x3B82F6,
    Secondary = 0x4F545C,
    Success = 0x22C55E,
    Danger = 0xED4245,
    Warning = 0xFBBD23,
    Azoxo = 0x5865F2,
    Green = 0x57F287,
    Yellow = 0xFEE75C,
    Fuchsia = 0xEB459E,
    Magic = 0xC026D3,
    Developer = 0x3E70DD,
    Balance = 0x45DDC0,
    Brilliance = 0xF07D5F,
    Nitro = 0xFF6BFA,
    Bravery = 0x9C84EF,
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
