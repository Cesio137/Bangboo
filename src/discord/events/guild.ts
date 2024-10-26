import { Event } from "#base";
import { res } from "#functions";
import { ChannelType } from "discord.js";

new Event({
    name: "Message edit logs",
    event: "guildMemberAdd",
    run(member) {
        const { guild } = member;
        const channel = guild.channels.cache.find(
            (ch) => ch.name === "😏┊welcome"
        );
        if (channel?.type !== ChannelType.GuildText) {
            console.log("Channel not found");
            return;
        }
        channel.send(res.green(`Welcome to the server, ${member.user}!`));
    },
});
