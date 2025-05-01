import { createEvent } from "#base";
import { Events } from "discord.js";
import { globalMessage } from "#functions";

createEvent({
    name: "Member Added",
    event: "guildMemberAdd",
    async run(member) {
        if (member.user.bot) {return}
        const { guild, user } = member;
        const system_channel = guild.systemChannel;
        if (!system_channel) {
            console.log("Channel not found");
            return;
        }
        globalMessage(Events.GuildMemberAdd, member, user, system_channel);
    },
});