import { createCommand } from "#base";
import { res } from "#functions";
import { ApplicationCommandOptionType, ApplicationCommandType, InteractionContextType } from "discord.js";
import { banCollector, deleteMessageCollector, kickCollector, timeoutCollector } from "./utils/index.js";

createCommand({
    name: "moderate",
    description: "Equality before the law is the cornerstone of justice ⚖.",
    type: ApplicationCommandType.ChatInput,
    contexts: [InteractionContextType.Guild],
    options: [
        {
            name: "action",
            description: "command option",
            required: true,
            type: ApplicationCommandOptionType.String,
            choices: [ 
                { name: "delete messages", value: "delete_messages" },
                { name: "timeout", value: "timeout" },
                { name: "kick", value: "kick" },
                { name: "ban", value: "ban" },
            ]
        }
    ],
    async run(interaction) {
        const { member, options } = interaction;
        if (!member.permissions.has("Administrator")) {
            interaction.reply(
                res.danger("You don't have **ADMINISTRATOR** permission.")
            )
        }
        
        switch(options.getString("action", true)) {
            case "delete_messages":
                deleteMessageCollector(interaction);
                break
            case "timeout":
                timeoutCollector(interaction);
                break
            case "kick":
                kickCollector(interaction);
                break
            case "ban":
                banCollector(interaction);
                break
        }
    }
});