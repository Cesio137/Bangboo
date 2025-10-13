use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Emojis {
    #[serde(rename = "static")]
    pub emojis_static: Static,

    pub animated: Animated,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Animated {
    pub boost: u64,

    pub icons_logo: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Static {
    pub identity: u64,

    pub action_check: u64,

    pub action_help: u64,

    pub action_info: u64,

    pub action_warning: u64,

    pub action_x: u64,

    pub check: u64,

    pub close: u64,

    pub arrow_down: u64,

    pub arrow_left: u64,

    pub arrow_refresh: u64,

    pub arrow_right: u64,

    pub arrow_up: u64,

    pub back: u64,

    pub next: u64,

    pub book_check: u64,

    pub book_minus: u64,

    pub book_plus: u64,

    pub book_x: u64,

    pub book: u64,

    pub bell_dot: u64,

    pub bell_minus: u64,

    pub bell_off: u64,

    pub bell_plus: u64,

    pub bell: u64,

    pub calendar_check: u64,

    pub calendar_cog: u64,

    pub calendar_days: u64,

    pub calendar_minus: u64,

    pub calendar_plus: u64,

    pub calendar_x: u64,

    pub camera_off: u64,

    pub camera: u64,

    pub add: u64,

    pub hammer: u64,

    pub minus: u64,

    pub clipboard_check: u64,

    pub clipboard_minus: u64,

    pub clipboard_plus: u64,

    pub clipboard_x: u64,

    pub clipboard: u64,

    pub clock_check: u64,

    pub clock_minus: u64,

    pub clock_off: u64,

    pub clock_plus: u64,

    pub clock: u64,

    pub cloud_cog: u64,

    pub cloud_download: u64,

    pub cloud_upload: u64,

    pub cloud: u64,

    pub code_braces: u64,

    pub code_brackets: u64,

    pub code_bug: u64,

    pub code_file_binary: u64,

    pub code_parentheses: u64,

    pub code_server_cog: u64,

    pub code_server_off: u64,

    pub code_server: u64,

    pub code_terminal: u64,

    pub code_window: u64,

    pub code_wrench: u64,

    pub database_backup: u64,

    pub database: u64,

    pub device_laptop: u64,

    pub device_pc: u64,

    pub device_smartphone: u64,

    pub device_tablet: u64,

    pub eye_off: u64,

    pub eye: u64,

    pub file_archive: u64,

    pub file_check: u64,

    pub file_cog: u64,

    pub file_files: u64,

    pub file_minus: u64,

    pub file_plus: u64,

    pub file_x: u64,

    pub file: u64,

    pub folder_archive: u64,

    pub folder_check: u64,

    pub folder_folders: u64,

    pub folder_minus: u64,

    pub folder_plus: u64,

    pub folder_x: u64,

    pub folder: u64,

    pub cpu: u64,

    pub ram: u64,

    pub ssd: u64,

    pub wifi: u64,

    pub headphone_off: u64,

    pub headphone: u64,

    pub image_download: u64,

    pub image_minus: u64,

    pub image_off: u64,

    pub image_plus: u64,

    pub image_upload: u64,

    pub image: u64,

    pub lock_unlock: u64,

    pub lock: u64,

    pub mail_check: u64,

    pub mail_minus: u64,

    pub mail_plus: u64,

    pub mail_x: u64,

    pub mail: u64,

    pub mic_off: u64,

    pub mic: u64,

    pub home: u64,

    pub id: u64,

    pub list: u64,

    pub other_brush: u64,

    pub other_cable: u64,

    pub other_crown: u64,

    pub other_dollar: u64,

    pub other_earth: u64,

    pub other_gauge: u64,

    pub other_gear: u64,

    pub other_graduation: u64,

    pub other_heart: u64,

    pub other_home: u64,

    pub other_save_off: u64,

    pub other_save: u64,

    pub other_text: u64,

    pub other_translate: u64,

    pub other_trash: u64,

    pub other_truck: u64,

    pub refresh: u64,

    pub phone_off: u64,

    pub phone: u64,

    pub pause: u64,

    pub resume: u64,

    pub stop: u64,

    pub view: u64,

    pub shield_check: u64,

    pub shield_minus: u64,

    pub shield_off: u64,

    pub shield_plus: u64,

    pub shield_x: u64,

    pub shield: u64,

    pub tag_tags: u64,

    pub tag: u64,

    pub ticket_check: u64,

    pub ticket_minus: u64,

    pub ticket_plus: u64,

    pub ticket_tickets: u64,

    pub ticket_x: u64,

    pub ticket: u64,

    pub timer_off: u64,

    pub timer_reset: u64,

    pub timer: u64,

    pub user_check: u64,

    pub user_cog: u64,

    pub user_minus: u64,

    pub user_plus: u64,

    pub user_users: u64,

    pub user_x: u64,

    pub user: u64,

    pub github: u64,

    pub instagram: u64,

    pub linkedin: u64,

    pub youtube: u64,

    pub eg_addemoji: u64,

    pub eg_addfile: u64,

    pub eg_addons: u64,

    pub eg_announcement: u64,

    pub eg_art: u64,

    pub eg_ask: u64,

    pub eg_ban: u64,

    pub eg_book: u64,

    pub eg_bot: u64,

    pub eg_calender: u64,

    pub eg_cautions: u64,

    pub eg_channels: u64,

    pub eg_cloud: u64,

    pub eg_clouddownload: u64,

    pub eg_cross: u64,

    pub eg_developers: u64,

    pub eg_discovery: u64,

    pub eg_downarrow: u64,

    pub eg_emojis: u64,

    pub eg_excl: u64,

    pub eg_female: u64,

    pub eg_files: u64,

    pub eg_fire: u64,

    pub eg_gift: u64,

    pub eg_globe: u64,

    pub eg_hammer: u64,

    pub eg_heart: u64,

    pub eg_home: u64,

    pub eg_hourclock: u64,

    pub eg_inbox: u64,

    pub eg_link: u64,

    pub eg_lock: u64,

    pub eg_mail: u64,

    pub eg_male: u64,

    pub eg_member: u64,

    pub eg_message: u64,

    pub eg_modadmin: u64,

    pub eg_monitor: u64,

    pub eg_music: u64,

    pub eg_netual: u64,

    pub eg_notification: u64,

    pub eg_openpage: u64,

    pub eg_pins: u64,

    pub eg_premium: u64,

    pub eg_question: u64,

    pub eg_refresh: u64,

    pub eg_right: u64,

    pub eg_setting: u64,

    pub eg_shield: u64,

    pub eg_star: u64,

    pub eg_stop: u64,

    pub eg_study: u64,

    pub eg_support: u64,

    pub eg_thumbdown: u64,

    pub eg_thumbup: u64,

    pub eg_ticket: u64,

    pub eg_tools: u64,

    pub eg_trans: u64,

    pub eg_unlock: u64,

    pub eg_uparrow: u64,

    pub eg_upleft: u64,

    pub eg_upload: u64,

    pub eg_upright: u64,

    pub eg_video: u64,

    pub eg_wave: u64,

    pub eg_wrench: u64,

    pub eg_wrong: u64,

    pub iconslogo: u64,

    pub icons_activedevbadge: u64,

    pub icons_activities: u64,

    pub icons_adventcalendar: u64,

    pub icons_announce: u64,

    pub icons_archive: u64,

    pub icons_audiodisable: u64,

    pub icons_audioenable: u64,

    pub icons_award: u64,

    pub icons_awardcup: u64,

    pub icons_backforward: u64,

    pub icons_badping: u64,

    pub icons_ban: u64,

    pub icons_bank: u64,

    pub icons_beta1: u64,

    #[serde(rename = "icons_beta1a")]
    pub icons_beta1_a: u64,

    pub icons_beta2: u64,

    #[serde(rename = "icons_beta2a")]
    pub icons_beta2_a: u64,

    pub icons_birdman: u64,

    pub icons_box: u64,

    pub icons_bright: u64,

    pub icons_bugs: u64,

    pub icons_bulb: u64,

    pub icons_calendar1: u64,

    pub icons_callconnect: u64,

    pub icons_calldecline: u64,

    pub icons_calldisconnect: u64,

    pub icons_channel: u64,

    pub icons_clock: u64,

    pub icons_coin: u64,

    pub icons_colorboostnitro: u64,

    pub icons_colornitro: u64,

    pub icons_colorserverpartner: u64,

    pub icons_colorserververified: u64,

    pub icons_colorstaff: u64,

    pub icons_connect: u64,

    pub icons_correct: u64,

    pub icons_creditcard: u64,

    pub icons_customstaff: u64,

    pub icons_dac: u64,

    pub icons_dblurple: u64,

    pub icons_delete: u64,

    pub icons_dfuchsia: u64,

    pub icons_dgreen: u64,

    pub icons_discover: u64,

    pub icons_djoin: u64,

    pub icons_dleave: u64,

    pub icons_dollar: u64,

    pub icons_download: u64,

    pub icons_downvote: u64,

    pub icons_dred: u64,

    pub icons_dwhite: u64,

    pub icons_dyellow: u64,

    pub icons_edit: u64,

    pub icons_emojiguardian: u64,

    pub icons_eventcolour: u64,

    pub icons_exclamation: u64,

    pub icons_file: u64,

    pub icons_fire: u64,

    pub icons_forum: u64,

    pub icons_forumlocked: u64,

    pub icons_forumnfsw: u64,

    pub icons_frontforward: u64,

    pub icons_gitbranch: u64,

    pub icons_gitcommit: u64,

    pub icons_gitmerge: u64,

    pub icons_gitpullrequest: u64,

    pub icons_globe: u64,

    pub icons_goodping: u64,

    pub icons_hammer: u64,

    pub icons_headphone: u64,

    pub icons_headphonedeafen: u64,

    pub icons_hyphen: u64,

    pub icons_idelping: u64,

    pub icons_illustrator: u64,

    pub icons_info: u64,

    pub icons_invite: u64,

    pub icons_join: u64,

    pub icons_kick: u64,

    pub icons_leave: u64,

    pub icons_link: u64,

    pub icons_linked: u64,

    pub icons_live: u64,

    pub icons_loading: u64,

    pub icons_magicwand: u64,

    pub icons_mashroomman: u64,

    pub icons_mentalhealth: u64,

    pub icons_mic: u64,

    pub icons_micmute: u64,

    pub icons_monitor: u64,

    pub icons_musicstop: u64,

    pub icons_new1: u64,

    #[serde(rename = "icons_new1a")]
    pub icons_new1_a: u64,

    pub icons_new2: u64,

    #[serde(rename = "icons_new2a")]
    pub icons_new2_a: u64,

    pub icons_newmembers: u64,

    pub icons_news1: u64,

    pub icons_news2: u64,

    pub icons_night: u64,

    pub icons_nitro: u64,

    pub icons_nitroboost: u64,

    pub icons_owner: u64,

    pub icons_paintpadbrush: u64,

    pub icons_pause: u64,

    pub icons_paypal: u64,

    pub icons_pen: u64,

    pub icons_people: u64,

    pub icons_person: u64,

    pub icons_photoshop: u64,

    pub icons_pin: u64,

    pub icons_ping: u64,

    pub icons_plant: u64,

    pub icons_play: u64,

    pub icons_plus: u64,

    pub icons_podcast: u64,

    pub icons_premiumchannel: u64,

    pub icons_reminder: u64,

    pub icons_repeat: u64,

    pub icons_repeatonce: u64,

    pub icons_reply: u64,

    pub icons_rightarrow: u64,

    pub icons_saturn: u64,

    pub icons_screenshare: u64,

    pub icons_search: u64,

    pub icons_sentry: u64,

    pub icons_servermute: u64,

    pub icons_settings: u64,

    pub icons_share: u64,

    pub icons_shine1: u64,

    pub icons_shine2: u64,

    pub icons_shine3: u64,

    pub icons_shuffle: u64,

    pub icons_splash: u64,

    pub icons_spotify: u64,

    pub icons_stageleave: u64,

    pub icons_stagelocked: u64,

    pub icons_stagemoderator: u64,

    pub icons_stagemoveaudience: u64,

    pub icons_stagemovespeaker: u64,

    pub icons_stagerequesttospeak: u64,

    pub icons_stagerequesttospeaklist: u64,

    pub icons_star: u64,

    pub icons_store: u64,

    pub icons_supportscommandsbadge: u64,

    pub icons_text1: u64,

    pub icons_text2: u64,

    pub icons_text3: u64,

    pub icons_text4: u64,

    pub icons_text5: u64,

    pub icons_text6: u64,

    pub icons_timeout: u64,

    pub icons_topgg: u64,

    pub icons_transferownership: u64,

    pub icons_update1: u64,

    pub icons_update2: u64,

    pub icons_upvote: u64,

    pub icons_verified: u64,

    pub icons_video: u64,

    pub icons_wrong: u64,

    pub icons_wumpus: u64,

    pub icons_xmarkwhite: u64,

    pub icons_bookmark: u64,

    pub icons_busy: u64,

    pub icons_camera: u64,

    pub icons_clouddown: u64,

    pub icons_code: u64,

    pub icons_control: u64,

    pub icons_downarrow: u64,

    pub icons_education: u64,

    pub icons_flag: u64,

    pub icons_folder: u64,

    pub icons_fword: u64,

    pub icons_games: u64,

    pub icons_gif: u64,

    pub icons_gift: u64,

    pub icons_heart: u64,

    pub icons_hi: u64,

    pub icons_id: u64,

    pub icons_idle: u64,

    pub icons_image: u64,

    pub icons_leftarrow: u64,

    pub icons_list: u64,

    pub icons_loadingerror: u64,

    pub icons_message: u64,

    pub icons_music: u64,

    pub icons_notify: u64,

    pub icons_off: u64,

    pub icons_offline: u64,

    pub icons_on: u64,

    pub icons_online: u64,

    pub icons_outage: u64,

    pub icons_premium: u64,

    pub icons_question: u64,

    pub icons_quotes: u64,

    pub icons_richpresence: u64,

    pub icons_rules: u64,

    pub icons_slashcmd: u64,

    pub icons_spark: u64,

    pub icons_speaker: u64,

    pub icons_speakerlock: u64,

    pub icons_speakerlow: u64,

    pub icons_speakermute: u64,

    pub icons_stickers: u64,

    pub icons_stream: u64,

    pub icons_ticket: u64,

    pub icons_tilde: u64,

    pub icons_todolist: u64,

    pub icons_uparrow: u64,

    pub icons_update: u64,

    pub icons_view: u64,

    pub icons_vip: u64,

    pub icons_1: u64,

    pub icons_addreactions: u64,

    pub icons_aka: u64,

    pub icons_behance: u64,

    pub icons_beta: u64,

    pub icons_bots: u64,

    pub icons_clean: u64,

    pub icons_defaultperms: u64,

    pub icons_discordbotdev: u64,

    pub icons_discordbughunter: u64,

    pub icons_discordhypesquard: u64,

    pub icons_discordmod: u64,

    pub icons_discordnitro: u64,

    pub icons_discordpartner: u64,

    pub icons_discordstaff: u64,

    pub icons_dislike: u64,

    pub icons_earlysupporter: u64,

    pub icons_fb: u64,

    pub icons_figma: u64,

    pub icons_files: u64,

    pub icons_friends: u64,

    pub icons_github: u64,

    pub icons_hoursglass: u64,

    pub icons_hsbalance: u64,

    pub icons_hsbravery: u64,

    pub icons_hsbrilliance: u64,

    pub icons_instagram: u64,

    pub icons_kicking: u64,

    pub icons_kofi: u64,

    pub icons_like: u64,

    pub icons_locked: u64,

    pub icons_loop: u64,

    pub icons_menu: u64,

    pub icons_msvisualcode: u64,

    pub icons_new: u64,

    pub icons_partner: u64,

    pub icons_patreon: u64,

    pub icons_pings: u64,

    pub icons_queue: u64,

    pub icons_reddit: u64,

    pub icons_serverpartner: u64,

    pub icons_serververified: u64,

    pub icons_snapchat: u64,

    pub icons_supportteam: u64,

    pub icons_twitter: u64,

    pub icons_unlock: u64,

    pub icons_youtube: u64,

    pub icons_banmembers: u64,

    pub icons_channelfollowed: u64,

    pub icons_createcategory: u64,

    pub icons_createchannel: u64,

    pub icons_createchannels: u64,

    pub icons_createemoji: u64,

    pub icons_createintegration: u64,

    pub icons_createrole: u64,

    pub icons_createsticker: u64,

    pub icons_createthread: u64,

    pub icons_createwebhook: u64,

    pub icons_deletechannel: u64,

    pub icons_deleteemoji: u64,

    pub icons_deleteevent: u64,

    pub icons_deleteintegration: u64,

    pub icons_deleterole: u64,

    pub icons_deletesticker: u64,

    pub icons_deletethread: u64,

    pub icons_deletewebhook: u64,

    pub icons_disable: u64,

    pub icons_discord: u64,

    pub icons_enable: u64,

    pub icons_endstage: u64,

    pub icons_envelope: u64,

    pub icons_generalinfo: u64,

    pub icons_growth: u64,

    pub icons_linkadd: u64,

    pub icons_linkrevoke: u64,

    pub icons_linkupdate: u64,

    pub icons_notificationsettings: u64,

    pub icons_oauth2: u64,

    pub icons_roles: u64,

    pub icons_scheduleevent: u64,

    pub icons_serverinsight: u64,

    pub icons_startstage: u64,

    pub icons_swardx: u64,

    pub icons_threadchannel: u64,

    pub icons_unbanmember: u64,

    pub icons_updatechannel: u64,

    pub icons_updateemoji: u64,

    pub icons_updateevent: u64,

    pub icons_updateintegration: u64,

    pub icons_updatemember: u64,

    pub icons_updaterole: u64,

    pub icons_updateserver: u64,

    pub icons_updatestage: u64,

    pub icons_updatesticker: u64,

    pub icons_updatethread: u64,

    pub icons_updatewebhook: u64,

    pub icons_0: u64,

    pub icons_10: u64,

    pub icons_2: u64,

    pub icons_3: u64,

    pub icons_4: u64,

    pub icons_5: u64,

    pub icons_6: u64,

    pub icons_7: u64,

    pub icons_8: u64,

    pub icons_9: u64,

    pub icons_a: u64,

    pub icons_amogus: u64,

    pub icons_b: u64,

    pub icons_bday: u64,

    pub icons_book: u64,

    pub icons_c: u64,

    pub icons_d: u64,

    pub icons_e: u64,

    pub icons_f: u64,

    pub icons_fingerprint: u64,

    pub icons_g: u64,

    pub icons_guardian: u64,

    pub icons_h: u64,

    pub icons_he_him: u64,

    pub icons_i: u64,

    pub icons_j: u64,

    pub icons_k: u64,

    pub icons_l: u64,

    pub icons_library: u64,

    pub icons_m: u64,

    pub icons_n: u64,

    pub icons_o: u64,

    pub icons_p: u64,

    pub icons_q: u64,

    pub icons_r: u64,

    pub icons_s: u64,

    pub icons_she_her: u64,

    pub icons_statsdown: u64,

    pub icons_t: u64,

    pub icons_tada: u64,

    pub icons_they_them: u64,

    pub icons_translate: u64,

    pub icons_u: u64,

    pub icons_v: u64,

    pub icons_vpn: u64,

    pub icons_w: u64,

    pub icons_x: u64,

    pub icons_y: u64,

    pub icons_z: u64,

    pub icons_18: u64,

    pub icons_bigender: u64,

    pub icons_calender: u64,

    pub icons_calenderdate: u64,

    pub icons_cmd: u64,

    pub icons_discordjs: u64,

    pub icons_female: u64,

    pub icons_gay: u64,

    pub icons_gender: u64,

    pub icons_hetero: u64,

    pub icons_jpg: u64,

    pub icons_js: u64,

    pub icons_lesbian: u64,

    pub icons_male: u64,

    pub icons_moderationhig: u64,

    pub icons_moderationhighest: u64,

    pub icons_moderationlow: u64,

    pub icons_moderationmedium: u64,

    pub icons_moderationnone: u64,

    pub icons_nodejs: u64,

    pub icons_png: u64,

    pub icons_radmins: u64,

    pub icons_rartists: u64,

    pub icons_rboosters: u64,

    pub icons_rbots: u64,

    pub icons_rcamera: u64,

    pub icons_rdevelopers: u64,

    pub icons_revents: u64,

    pub icons_rfire: u64,

    pub icons_rguardians: u64,

    pub icons_rhelpers: u64,

    pub icons_rmembers: u64,

    pub icons_rmods: u64,

    pub icons_rowner: u64,

    pub icons_rpodcast: u64,

    pub icons_rsdonator: u64,

    pub icons_rspartner: u64,

    pub icons_rsstaffs: u64,

    pub icons_rstaff: u64,

    pub icons_rverification: u64,

    pub icons_rverified: u64,

    pub icons_rvip: u64,

    pub icons_snowflake: u64,

    pub icons_tiktok: u64,

    pub icons_transgender: u64,

    pub icons_twitch: u64,

    pub icons_vklogo: u64,

    pub icons_warning: u64,

    pub icons_wave: u64,

    pub icons_webp: u64,
}
