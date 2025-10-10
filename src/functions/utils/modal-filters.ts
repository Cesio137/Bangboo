import { Guild } from "discord.js";

export function filterUsers(ids: string[], guild: Guild): string[] {
    let users: string[] = [];
    if (ids.length < 1) { return users; }
    for (const id of ids) {
        const member = guild.members.cache.get(id);
        if (!member) {continue}
        if (!member.user.bot && !member.permissions.has("Administrator") && !member.roles.cache.has(guildData.roles.stf)) {
            users.push(member.id);
        }
    }

    return users;
}