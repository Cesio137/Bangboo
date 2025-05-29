import { createCommand } from "#base";
import { icon } from "#functions";
import { createEmbed } from "@magicyan/discord";
import { ApplicationCommandType } from "discord.js";

createCommand({
    name: "social",
    description: "Social medias",
    type: ApplicationCommandType.ChatInput,
    async run(interaction) {
        const socials = [
            `${icon.youtube} **[Youtube](https://www.youtube.com/@NathanMiguel1)**`,
            `${icon.instagram} **[Instagram](https://www.instagram.com/nathan_cmiguel/)**`,
            `${icon.github} **[Github](https://github.com/Cesio137)**`,
            `${icon.linkedin} **[Linkedin](https://www.linkedin.com/in/nathan-miguel-488b462b1/)**`,
        ];

        const embed = createEmbed({
            color: "Green",
            title: "Social medias",
            description: `${socials.join('\n')}`,
        });

        interaction.reply({ embeds: [embed] });
    }
});