import { Command } from "#base";
import { res } from "#functions";
import { ApplicationCommandOptionType, ApplicationCommandType, Events } from "discord.js";

new Command({
    name: "trigger",
    description: "Trigger a event to test bot.",
    type: ApplicationCommandType.ChatInput,
    options: [
        {
            name: "event",
            description: "Event name",
            type: ApplicationCommandOptionType.String,
            required: true,
            choices: [
                {
                    name: Events.GuildMemberAdd,
                    value: Events.GuildMemberAdd,
                },
                {
                    name: Events.GuildMemberRemove,
                    value: Events.GuildMemberRemove,
                },
            ],
        },
    ],
    async run(interaction) {
        const { client, options, user, guild } = interaction;
        const event = options.getString("event");
        if (!event) {
            interaction.reply(res.danger("Error trying to get event name."));
            return;
        }
        const member = guild.members.cache.get(user.id);
        if (!member) {
            interaction.reply(res.danger("Error trying to get a valid user."));
            return;
        }
        client.emit(event, member);
        interaction.reply(res.green(`Event ${event} triggered for member ${member.displayName}`));
    },
});
