import { createEvent } from "#base";
import { globalMessage, logger } from "#functions";
import { Events } from "discord.js";

createEvent({
    name: "Ban Added",
    event: "guildBanAdd",
    async run(ban) {
        const { guild, user } = ban;
        const system_channel = guild.systemChannel;
        if (!system_channel) {
            logger.error("System channel not found");
            return;
        }
        globalMessage(Events.GuildBanAdd, undefined, user, system_channel);
    }
});