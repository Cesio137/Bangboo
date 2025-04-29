import { createEvent } from "#base";
import { globalMessage } from "#functions";
import { Events } from "discord.js";

createEvent({
    name: "Member Removed",
    event: "guildMemberRemove",
    async run(member) {
        const { guild } = member;
        const system_channel = guild.systemChannel;
        if (typeof system_channel == null) {
            console.log("Channel not found");
            return;
        }
        globalMessage(Events.GuildMemberRemove, member, system_channel!);
    },
});