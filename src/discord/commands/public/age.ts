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
        const { displayName, createdAt } = user;
        const date = createdAt.toUTCString().split(" ");
        const age = `**${(displayName)}**'s account was created at ${date[1]}/${date[2]}/${date[3]} ${date[4]}`;
        interaction.reply(res.green(age));
    }
});