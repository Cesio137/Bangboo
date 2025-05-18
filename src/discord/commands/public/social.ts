import { createCommand } from "#base";
import { createEmbed } from "@magicyan/discord";
import { ApplicationCommandType } from "discord.js";

createCommand({
    name: "social",
    description: "Social medias",
    type: ApplicationCommandType.ChatInput,
    async run(interaction) {
        const socials = [
            `**[Youtube](https://www.youtube.com/@NathanMiguel1)**`,
            `**[Instagram](https://www.instagram.com/nathan_cmiguel/)**`,
            `**[Github](https://github.com/Cesio137)**`,
            `**[Linkedin](https://www.linkedin.com/in/nathan-miguel-488b462b1/)**`,
        ];

        const embed = createEmbed({
            color: "Green",
            title: "Social medias",
            description: `${socials.join('\n')}`,
        });

        interaction.reply({ embeds: [embed] });
    }
});