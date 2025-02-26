import { createEvent } from "#base";
import { globalMessage } from "#functions";
import { ChannelType, Events } from "discord.js";

createEvent({
    name: "Member Added",
    event: "guildMemberAdd",
    run(member) {
        const { guild } = member;
        const channel = guild.channels.cache.find((ch) => ch.name === "😏┊welcome");
        if (channel?.type !== ChannelType.GuildText) {
            console.log("Channel not found");
            return;
        }
        globalMessage(Events.GuildMemberAdd, member);
    },
});