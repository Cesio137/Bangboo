import { createCommand } from "#base";
import { res } from "#functions";
import { ApplicationCommandOptionType, ApplicationCommandType, InteractionContextType } from "discord.js";

createCommand({
    name: "age",
    description: "Displays your or another user's account creation date.",
    type: ApplicationCommandType.ChatInput,
    contexts: [ InteractionContextType.Guild ],
    options: [
        {
            name: "user",
            type: ApplicationCommandOptionType.User,
            description: "Selected user.",
        },
    ],
    async run(interaction) {
        const { options } = interaction;
        const user = options.getUser("user") || interaction.user;
        const { displayName, createdTimestamp,  } = user;
        let age = "";
        if (interaction.locale == "pt-BR") {
            age = `**${(displayName)}** criou a conta <t:${Math.floor(createdTimestamp / 1000)}:R> em um(a) <t:${Math.floor(createdTimestamp / 1000)}:F>`;
        } else {
            age = `**${(displayName)}**'s account was created <t:${Math.floor(createdTimestamp / 1000)}:R> on <t:${Math.floor(createdTimestamp / 1000)}:F>`;
        }
        interaction.reply(res.green(age));
    }
});