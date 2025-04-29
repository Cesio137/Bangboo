import { createEvent } from "#base";
import { Events } from "discord.js";
import { globalMessage } from "#functions";

createEvent({
    name: "Member Added",
    event: "guildMemberAdd",
    async run(member) {
        const { guild } = member;
        const system_channel = guild.systemChannel;
        if (typeof system_channel == null) {
            console.log("Channel not found");
            return;
        }
        globalMessage(Events.GuildMemberAdd, member, system_channel!);
    },
});