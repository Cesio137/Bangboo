import { createEvent } from "#base";
import { banMessage } from "#functions";
import { Events } from "discord.js";

createEvent({
    name: "Ban Added",
    event: "guildBanAdd",
    async run(ban) {
        const { guild, user } = ban;
        const system_channel = guild.systemChannel;
        if (!system_channel) {
            console.log("Channel not found");
             return;
        }
        banMessage(Events.GuildMemberRemove, user, system_channel);
    }
});