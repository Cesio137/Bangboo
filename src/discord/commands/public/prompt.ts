import { createCommand } from "#base";
import { icon } from "#functions";
import { settings } from "#settings";
import { gemini } from "#tools";
import { createEmbed } from "@magicyan/discord";
import { ApplicationCommandOptionType, ApplicationCommandType } from "discord.js";

createCommand({
    name: "prompt",
    description: "Choose a IA and interact with it!",
    type: ApplicationCommandType.ChatInput,
    options: [
        {
            name: "text",
            description: "Enter the text.",
            type: ApplicationCommandOptionType.String,
        },
    ],
    async run(interaction){
        const { options } = interaction;

        interaction.deferReply({ flags: [ "Ephemeral" ] });

        const text = options.getString("text", true);

        const { response } = await gemini.text.generateContent(text);
        const result = gemini.getText(response);

        if (!result.success || !result.text) {
            const embed = createEmbed({
                color: settings.colors.danger,
                description: `${icon.close} An unspected error happen!`,
            });
            interaction.editReply({ embeds: [embed] });
            return;
        }

        const maxLength = 3000;
        const texts: string[] = [];

        for (let i = 0; i < result.text.length; i += maxLength) {
            texts.push(result.text.slice(i, i + maxLength));
        }

        const embed = createEmbed({
            color: settings.colors.success,
            description: texts.shift(),
        });

        await interaction.editReply({ embeds: [embed] });
        if (texts.length < 1) return;

        while (text.length > 0) {
            const description = texts.shift();
            if (typeof description === "undefined") break;
            const embed = createEmbed({
                color: settings.colors.success,
                description,
            });

            await interaction.followUp({ flags: [ "Ephemeral" ], embeds: [embed] });
        }
    }
});