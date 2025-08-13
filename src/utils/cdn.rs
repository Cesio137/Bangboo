pub fn display_avatar_url(user_id: u64, hash: &str, size: u16) -> String {
    if size == 0 {
        format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png",
            user_id, hash
        )
    } else {
        format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png?size={}",
            user_id, hash, size
        )
    }
}
