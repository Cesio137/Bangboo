import { createEvent } from "#base";
import { Events } from "discord.js";
import { globalMessage, logger } from "#functions";

createEvent({
    name: "Member Removed",
    event: "guildMemberRemove",
    async run(member) {
        if (member.user.bot) return;
        const { guild, user } = member;
        if (guild.bans.cache.has(user.id)) return;
        const system_channel = guild.systemChannel;
        if (!system_channel) {
            logger.error("System channel not found");
            return;
        }
        globalMessage(Events.GuildMemberRemove, member, user, system_channel);
    },
});