import { Command } from "#base";
import {
    ApplicationCommandOptionType,
    ApplicationCommandType,
    AttachmentBuilder,
} from "discord.js";

new Command({
    name: "emojis",
    description: "Show emotes list.",
    type: ApplicationCommandType.ChatInput,
    options: [
        {
            name: "origin",
            description: "Emotes origin.",
            type: ApplicationCommandOptionType.String,
            choices: [
                { name: "server", value: "guild" },
                { name: "bot", value: "client" },
            ],
        },
    ],
    async run(interaction) {
        const { options } = interaction;
        const source = (options.getString("origem") ?? "guild") as "client" | "guild";
        const emoji = interaction[source].emojis.cache;
        const [animatedEmojis, staticEmojis] = emoji.partition((e) => !!e.animated);
        const json = {
            animated: animatedEmojis.reduce(
                (obj, { id, name }) => Object.assign(obj, { [name ?? id]: id }),
                {}
            ),
            static: staticEmojis.reduce(
                (obj, { id, name }) => Object.assign(obj, { [name ?? id]: id }),
                {}
            ),
        };
        const buffer = Buffer.from(JSON.stringify(json, null, 2));
        const attachment = new AttachmentBuilder(buffer, { name: "emojis.json" });
        interaction.reply({ ephemeral, content: "Emojis list.", files: [attachment] });
    },
});
