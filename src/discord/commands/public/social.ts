import { createCommand } from "#base";
import { socialComponent } from "#menus";
import { ApplicationCommandType } from "discord.js";

createCommand({
    name: "social",
    description: "Social medias",
    type: ApplicationCommandType.ChatInput,
    async run(interaction) {
        const component = socialComponent();
        interaction.reply({ flags: [ "IsComponentsV2", "Ephemeral" ], components: [ component ] });
    }
});