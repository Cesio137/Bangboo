// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Emojis;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Emojis = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Emojis {
    #[serde(rename = "static")]
    pub emojis_static: Static,

    pub animated: Animated,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Animated {
    pub boost: String,

    pub icons_logo: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Static {
    pub identity: String,

    pub action_check: String,

    pub action_help: String,

    pub action_info: String,

    pub action_warning: String,

    pub action_x: String,

    pub check: String,

    pub close: String,

    pub arrow_down: String,

    pub arrow_left: String,

    pub arrow_refresh: String,

    pub arrow_right: String,

    pub arrow_up: String,

    pub back: String,

    pub next: String,

    pub book_check: String,

    pub book_minus: String,

    pub book_plus: String,

    pub book_x: String,

    pub book: String,

    pub bell_dot: String,

    pub bell_minus: String,

    pub bell_off: String,

    pub bell_plus: String,

    pub bell: String,

    pub calendar_check: String,

    pub calendar_cog: String,

    pub calendar_days: String,

    pub calendar_minus: String,

    pub calendar_plus: String,

    pub calendar_x: String,

    pub camera_off: String,

    pub camera: String,

    pub add: String,

    pub hammer: String,

    pub minus: String,

    pub clipboard_check: String,

    pub clipboard_minus: String,

    pub clipboard_plus: String,

    pub clipboard_x: String,

    pub clipboard: String,

    pub clock_check: String,

    pub clock_minus: String,

    pub clock_off: String,

    pub clock_plus: String,

    pub clock: String,

    pub cloud_cog: String,

    pub cloud_download: String,

    pub cloud_upload: String,

    pub cloud: String,

    pub code_braces: String,

    pub code_brackets: String,

    pub code_bug: String,

    pub code_file_binary: String,

    pub code_parentheses: String,

    pub code_server_cog: String,

    pub code_server_off: String,

    pub code_server: String,

    pub code_terminal: String,

    pub code_window: String,

    pub code_wrench: String,

    pub database_backup: String,

    pub database: String,

    pub device_laptop: String,

    pub device_pc: String,

    pub device_smartphone: String,

    pub device_tablet: String,

    pub eye_off: String,

    pub eye: String,

    pub file_archive: String,

    pub file_check: String,

    pub file_cog: String,

    pub file_files: String,

    pub file_minus: String,

    pub file_plus: String,

    pub file_x: String,

    pub file: String,

    pub folder_archive: String,

    pub folder_check: String,

    pub folder_folders: String,

    pub folder_minus: String,

    pub folder_plus: String,

    pub folder_x: String,

    pub folder: String,

    pub cpu: String,

    pub ram: String,

    pub ssd: String,

    pub wifi: String,

    pub headphone_off: String,

    pub headphone: String,

    pub image_download: String,

    pub image_minus: String,

    pub image_off: String,

    pub image_plus: String,

    pub image_upload: String,

    pub image: String,

    pub lock_unlock: String,

    pub lock: String,

    pub mail_check: String,

    pub mail_minus: String,

    pub mail_plus: String,

    pub mail_x: String,

    pub mail: String,

    pub mic_off: String,

    pub mic: String,

    pub home: String,

    pub id: String,

    pub list: String,

    pub other_brush: String,

    pub other_cable: String,

    pub other_crown: String,

    pub other_dollar: String,

    pub other_earth: String,

    pub other_gauge: String,

    pub other_gear: String,

    pub other_graduation: String,

    pub other_heart: String,

    pub other_home: String,

    pub other_save_off: String,

    pub other_save: String,

    pub other_text: String,

    pub other_translate: String,

    pub other_trash: String,

    pub other_truck: String,

    pub refresh: String,

    pub phone_off: String,

    pub phone: String,

    pub pause: String,

    pub resume: String,

    pub stop: String,

    pub view: String,

    pub shield_check: String,

    pub shield_minus: String,

    pub shield_off: String,

    pub shield_plus: String,

    pub shield_x: String,

    pub shield: String,

    pub tag_tags: String,

    pub tag: String,

    pub ticket_check: String,

    pub ticket_minus: String,

    pub ticket_plus: String,

    pub ticket_tickets: String,

    pub ticket_x: String,

    pub ticket: String,

    pub timer_off: String,

    pub timer_reset: String,

    pub timer: String,

    pub user_check: String,

    pub user_cog: String,

    pub user_minus: String,

    pub user_plus: String,

    pub user_users: String,

    pub user_x: String,

    pub user: String,

    pub github: String,

    pub instagram: String,

    pub linkedin: String,

    pub youtube: String,

    pub eg_addemoji: String,

    pub eg_addfile: String,

    pub eg_addons: String,

    pub eg_announcement: String,

    pub eg_art: String,

    pub eg_ask: String,

    pub eg_ban: String,

    pub eg_book: String,

    pub eg_bot: String,

    pub eg_calender: String,

    pub eg_cautions: String,

    pub eg_channels: String,

    pub eg_cloud: String,

    pub eg_clouddownload: String,

    pub eg_cross: String,

    pub eg_developers: String,

    pub eg_discovery: String,

    pub eg_downarrow: String,

    pub eg_emojis: String,

    pub eg_excl: String,

    pub eg_female: String,

    pub eg_files: String,

    pub eg_fire: String,

    pub eg_gift: String,

    pub eg_globe: String,

    pub eg_hammer: String,

    pub eg_heart: String,

    pub eg_home: String,

    pub eg_hourclock: String,

    pub eg_inbox: String,

    pub eg_link: String,

    pub eg_lock: String,

    pub eg_mail: String,

    pub eg_male: String,

    pub eg_member: String,

    pub eg_message: String,

    pub eg_modadmin: String,

    pub eg_monitor: String,

    pub eg_music: String,

    pub eg_netual: String,

    pub eg_notification: String,

    pub eg_openpage: String,

    pub eg_pins: String,

    pub eg_premium: String,

    pub eg_question: String,

    pub eg_refresh: String,

    pub eg_right: String,

    pub eg_setting: String,

    pub eg_shield: String,

    pub eg_star: String,

    pub eg_stop: String,

    pub eg_study: String,

    pub eg_support: String,

    pub eg_thumbdown: String,

    pub eg_thumbup: String,

    pub eg_ticket: String,

    pub eg_tools: String,

    pub eg_trans: String,

    pub eg_unlock: String,

    pub eg_uparrow: String,

    pub eg_upleft: String,

    pub eg_upload: String,

    pub eg_upright: String,

    pub eg_video: String,

    pub eg_wave: String,

    pub eg_wrench: String,

    pub eg_wrong: String,

    pub iconslogo: String,

    pub icons_activedevbadge: String,

    pub icons_activities: String,

    pub icons_adventcalendar: String,

    pub icons_announce: String,

    pub icons_archive: String,

    pub icons_audiodisable: String,

    pub icons_audioenable: String,

    pub icons_award: String,

    pub icons_awardcup: String,

    pub icons_backforward: String,

    pub icons_badping: String,

    pub icons_ban: String,

    pub icons_bank: String,

    pub icons_beta1: String,

    #[serde(rename = "icons_beta1a")]
    pub icons_beta1_a: String,

    pub icons_beta2: String,

    #[serde(rename = "icons_beta2a")]
    pub icons_beta2_a: String,

    pub icons_birdman: String,

    pub icons_box: String,

    pub icons_bright: String,

    pub icons_bugs: String,

    pub icons_bulb: String,

    pub icons_calendar1: String,

    pub icons_callconnect: String,

    pub icons_calldecline: String,

    pub icons_calldisconnect: String,

    pub icons_channel: String,

    pub icons_clock: String,

    pub icons_coin: String,

    pub icons_colorboostnitro: String,

    pub icons_colornitro: String,

    pub icons_colorserverpartner: String,

    pub icons_colorserververified: String,

    pub icons_colorstaff: String,

    pub icons_connect: String,

    pub icons_correct: String,

    pub icons_creditcard: String,

    pub icons_customstaff: String,

    pub icons_dac: String,

    pub icons_dblurple: String,

    pub icons_delete: String,

    pub icons_dfuchsia: String,

    pub icons_dgreen: String,

    pub icons_discover: String,

    pub icons_djoin: String,

    pub icons_dleave: String,

    pub icons_dollar: String,

    pub icons_download: String,

    pub icons_downvote: String,

    pub icons_dred: String,

    pub icons_dwhite: String,

    pub icons_dyellow: String,

    pub icons_edit: String,

    pub icons_emojiguardian: String,

    pub icons_eventcolour: String,

    pub icons_exclamation: String,

    pub icons_file: String,

    pub icons_fire: String,

    pub icons_forum: String,

    pub icons_forumlocked: String,

    pub icons_forumnfsw: String,

    pub icons_frontforward: String,

    pub icons_gitbranch: String,

    pub icons_gitcommit: String,

    pub icons_gitmerge: String,

    pub icons_gitpullrequest: String,

    pub icons_globe: String,

    pub icons_goodping: String,

    pub icons_hammer: String,

    pub icons_headphone: String,

    pub icons_headphonedeafen: String,

    pub icons_hyphen: String,

    pub icons_idelping: String,

    pub icons_illustrator: String,

    pub icons_info: String,

    pub icons_invite: String,

    pub icons_join: String,

    pub icons_kick: String,

    pub icons_leave: String,

    pub icons_link: String,

    pub icons_linked: String,

    pub icons_live: String,

    pub icons_loading: String,

    pub icons_magicwand: String,

    pub icons_mashroomman: String,

    pub icons_mentalhealth: String,

    pub icons_mic: String,

    pub icons_micmute: String,

    pub icons_monitor: String,

    pub icons_musicstop: String,

    pub icons_new1: String,

    #[serde(rename = "icons_new1a")]
    pub icons_new1_a: String,

    pub icons_new2: String,

    #[serde(rename = "icons_new2a")]
    pub icons_new2_a: String,

    pub icons_newmembers: String,

    pub icons_news1: String,

    pub icons_news2: String,

    pub icons_night: String,

    pub icons_nitro: String,

    pub icons_nitroboost: String,

    pub icons_owner: String,

    pub icons_paintpadbrush: String,

    pub icons_pause: String,

    pub icons_paypal: String,

    pub icons_pen: String,

    pub icons_people: String,

    pub icons_person: String,

    pub icons_photoshop: String,

    pub icons_pin: String,

    pub icons_ping: String,

    pub icons_plant: String,

    pub icons_play: String,

    pub icons_plus: String,

    pub icons_podcast: String,

    pub icons_premiumchannel: String,

    pub icons_reminder: String,

    pub icons_repeat: String,

    pub icons_repeatonce: String,

    pub icons_reply: String,

    pub icons_rightarrow: String,

    pub icons_saturn: String,

    pub icons_screenshare: String,

    pub icons_search: String,

    pub icons_sentry: String,

    pub icons_servermute: String,

    pub icons_settings: String,

    pub icons_share: String,

    pub icons_shine1: String,

    pub icons_shine2: String,

    pub icons_shine3: String,

    pub icons_shuffle: String,

    pub icons_splash: String,

    pub icons_spotify: String,

    pub icons_stageleave: String,

    pub icons_stagelocked: String,

    pub icons_stagemoderator: String,

    pub icons_stagemoveaudience: String,

    pub icons_stagemovespeaker: String,

    pub icons_stagerequesttospeak: String,

    pub icons_stagerequesttospeaklist: String,

    pub icons_star: String,

    pub icons_store: String,

    pub icons_supportscommandsbadge: String,

    pub icons_text1: String,

    pub icons_text2: String,

    pub icons_text3: String,

    pub icons_text4: String,

    pub icons_text5: String,

    pub icons_text6: String,

    pub icons_timeout: String,

    pub icons_topgg: String,

    pub icons_transferownership: String,

    pub icons_update1: String,

    pub icons_update2: String,

    pub icons_upvote: String,

    pub icons_verified: String,

    pub icons_video: String,

    pub icons_wrong: String,

    pub icons_wumpus: String,

    pub icons_xmarkwhite: String,

    pub icons_bookmark: String,

    pub icons_busy: String,

    pub icons_camera: String,

    pub icons_clouddown: String,

    pub icons_code: String,

    pub icons_control: String,

    pub icons_downarrow: String,

    pub icons_education: String,

    pub icons_flag: String,

    pub icons_folder: String,

    pub icons_fword: String,

    pub icons_games: String,

    pub icons_gif: String,

    pub icons_gift: String,

    pub icons_heart: String,

    pub icons_hi: String,

    pub icons_id: String,

    pub icons_idle: String,

    pub icons_image: String,

    pub icons_leftarrow: String,

    pub icons_list: String,

    pub icons_loadingerror: String,

    pub icons_message: String,

    pub icons_music: String,

    pub icons_notify: String,

    pub icons_off: String,

    pub icons_offline: String,

    pub icons_on: String,

    pub icons_online: String,

    pub icons_outage: String,

    pub icons_premium: String,

    pub icons_question: String,

    pub icons_quotes: String,

    pub icons_richpresence: String,

    pub icons_rules: String,

    pub icons_slashcmd: String,

    pub icons_spark: String,

    pub icons_speaker: String,

    pub icons_speakerlock: String,

    pub icons_speakerlow: String,

    pub icons_speakermute: String,

    pub icons_stickers: String,

    pub icons_stream: String,

    pub icons_ticket: String,

    pub icons_tilde: String,

    pub icons_todolist: String,

    pub icons_uparrow: String,

    pub icons_update: String,

    pub icons_view: String,

    pub icons_vip: String,

    pub icons_1: String,

    pub icons_addreactions: String,

    pub icons_aka: String,

    pub icons_behance: String,

    pub icons_beta: String,

    pub icons_bots: String,

    pub icons_clean: String,

    pub icons_defaultperms: String,

    pub icons_discordbotdev: String,

    pub icons_discordbughunter: String,

    pub icons_discordhypesquard: String,

    pub icons_discordmod: String,

    pub icons_discordnitro: String,

    pub icons_discordpartner: String,

    pub icons_discordstaff: String,

    pub icons_dislike: String,

    pub icons_earlysupporter: String,

    pub icons_fb: String,

    pub icons_figma: String,

    pub icons_files: String,

    pub icons_friends: String,

    pub icons_github: String,

    pub icons_hoursglass: String,

    pub icons_hsbalance: String,

    pub icons_hsbravery: String,

    pub icons_hsbrilliance: String,

    pub icons_instagram: String,

    pub icons_kicking: String,

    pub icons_kofi: String,

    pub icons_like: String,

    pub icons_locked: String,

    pub icons_loop: String,

    pub icons_menu: String,

    pub icons_msvisualcode: String,

    pub icons_new: String,

    pub icons_partner: String,

    pub icons_patreon: String,

    pub icons_pings: String,

    pub icons_queue: String,

    pub icons_reddit: String,

    pub icons_serverpartner: String,

    pub icons_serververified: String,

    pub icons_snapchat: String,

    pub icons_supportteam: String,

    pub icons_twitter: String,

    pub icons_unlock: String,

    pub icons_youtube: String,

    pub icons_banmembers: String,

    pub icons_channelfollowed: String,

    pub icons_createcategory: String,

    pub icons_createchannel: String,

    pub icons_createchannels: String,

    pub icons_createemoji: String,

    pub icons_createintegration: String,

    pub icons_createrole: String,

    pub icons_createsticker: String,

    pub icons_createthread: String,

    pub icons_createwebhook: String,

    pub icons_deletechannel: String,

    pub icons_deleteemoji: String,

    pub icons_deleteevent: String,

    pub icons_deleteintegration: String,

    pub icons_deleterole: String,

    pub icons_deletesticker: String,

    pub icons_deletethread: String,

    pub icons_deletewebhook: String,

    pub icons_disable: String,

    pub icons_discord: String,

    pub icons_enable: String,

    pub icons_endstage: String,

    pub icons_envelope: String,

    pub icons_generalinfo: String,

    pub icons_growth: String,

    pub icons_linkadd: String,

    pub icons_linkrevoke: String,

    pub icons_linkupdate: String,

    pub icons_notificationsettings: String,

    pub icons_oauth2: String,

    pub icons_roles: String,

    pub icons_scheduleevent: String,

    pub icons_serverinsight: String,

    pub icons_startstage: String,

    pub icons_swardx: String,

    pub icons_threadchannel: String,

    pub icons_unbanmember: String,

    pub icons_updatechannel: String,

    pub icons_updateemoji: String,

    pub icons_updateevent: String,

    pub icons_updateintegration: String,

    pub icons_updatemember: String,

    pub icons_updaterole: String,

    pub icons_updateserver: String,

    pub icons_updatestage: String,

    pub icons_updatesticker: String,

    pub icons_updatethread: String,

    pub icons_updatewebhook: String,

    pub icons_0: String,

    pub icons_10: String,

    pub icons_2: String,

    pub icons_3: String,

    pub icons_4: String,

    pub icons_5: String,

    pub icons_6: String,

    pub icons_7: String,

    pub icons_8: String,

    pub icons_9: String,

    pub icons_a: String,

    pub icons_amogus: String,

    pub icons_b: String,

    pub icons_bday: String,

    pub icons_book: String,

    pub icons_c: String,

    pub icons_d: String,

    pub icons_e: String,

    pub icons_f: String,

    pub icons_fingerprint: String,

    pub icons_g: String,

    pub icons_guardian: String,

    pub icons_h: String,

    pub icons_he_him: String,

    pub icons_i: String,

    pub icons_j: String,

    pub icons_k: String,

    pub icons_l: String,

    pub icons_library: String,

    pub icons_m: String,

    pub icons_n: String,

    pub icons_o: String,

    pub icons_p: String,

    pub icons_q: String,

    pub icons_r: String,

    pub icons_s: String,

    pub icons_she_her: String,

    pub icons_statsdown: String,

    pub icons_t: String,

    pub icons_tada: String,

    pub icons_they_them: String,

    pub icons_translate: String,

    pub icons_u: String,

    pub icons_v: String,

    pub icons_vpn: String,

    pub icons_w: String,

    pub icons_x: String,

    pub icons_y: String,

    pub icons_z: String,

    pub icons_18: String,

    pub icons_bigender: String,

    pub icons_calender: String,

    pub icons_calenderdate: String,

    pub icons_cmd: String,

    pub icons_discordjs: String,

    pub icons_female: String,

    pub icons_gay: String,

    pub icons_gender: String,

    pub icons_hetero: String,

    pub icons_jpg: String,

    pub icons_js: String,

    pub icons_lesbian: String,

    pub icons_male: String,

    pub icons_moderationhig: String,

    pub icons_moderationhighest: String,

    pub icons_moderationlow: String,

    pub icons_moderationmedium: String,

    pub icons_moderationnone: String,

    pub icons_nodejs: String,

    pub icons_png: String,

    pub icons_radmins: String,

    pub icons_rartists: String,

    pub icons_rboosters: String,

    pub icons_rbots: String,

    pub icons_rcamera: String,

    pub icons_rdevelopers: String,

    pub icons_revents: String,

    pub icons_rfire: String,

    pub icons_rguardians: String,

    pub icons_rhelpers: String,

    pub icons_rmembers: String,

    pub icons_rmods: String,

    pub icons_rowner: String,

    pub icons_rpodcast: String,

    pub icons_rsdonator: String,

    pub icons_rspartner: String,

    pub icons_rsstaffs: String,

    pub icons_rstaff: String,

    pub icons_rverification: String,

    pub icons_rverified: String,

    pub icons_rvip: String,

    pub icons_snowflake: String,

    pub icons_tiktok: String,

    pub icons_transgender: String,

    pub icons_twitch: String,

    pub icons_vklogo: String,

    pub icons_warning: String,

    pub icons_wave: String,

    pub icons_webp: String,
}
