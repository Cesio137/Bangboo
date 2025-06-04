import { createCommand } from "#base";
import { fabComponent } from "#menus";
import { ApplicationCommandType, InteractionContextType } from "discord.js";

createCommand({
    name: "fab",
    description: "Products on fab marketplace",
    contexts: [ InteractionContextType.Guild ],
    type: ApplicationCommandType.ChatInput,
    async run(interaction) {
        const component = fabComponent();

        interaction.reply({ flags: [ "IsComponentsV2", "Ephemeral" ], components: [ component ] });
    }
})