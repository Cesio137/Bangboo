import { Event } from "#base";
import { ChannelType, Events } from "discord.js";
import { globalMessage } from "#functions";

new Event({
    name: "Member Remove",
    event: "guildMemberRemove",
    run(member) {
        const { guild } = member;
        const channel = guild.channels.cache.find((ch) => ch.name === "😏┊welcome");
        if (channel?.type !== ChannelType.GuildText) {
            console.log("Channel not found");
            return;
        }
        globalMessage(Events.GuildMemberRemove, member);
    },
});
