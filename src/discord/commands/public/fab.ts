import { createCommand } from "#base";
import { products } from "#functions";
import { createEmbed } from "@magicyan/discord";
import { ApplicationCommandOptionType, ApplicationCommandType, InteractionContextType } from "discord.js";

createCommand({
    name: "fab",
    description: "Products on fab marketplace",
    contexts: [ InteractionContextType.Guild ],
    type: ApplicationCommandType.ChatInput,
    options: [
        {
            name: "products",
            description: "Select a product",
            type: ApplicationCommandOptionType.String,
            required: true,
            choices: [ 
                { name: "Engine User Settings", value: "Engine User Settings" },
                { name: "Internet Protocol", value: "Internet Protocol" }
            ]
        }
    ],
    async run(interaction) {
        const { options } = interaction;
        const product = options.getString("products", true);

        let description = "";
        let links = "";
        let thumb_link = "";

        switch (product) {
            case "Engine User Settings":
                description = products["engine-user-setings"]["product-desc"]
                links = `**[Buy it on fab](${products["engine-user-setings"]["product-url"]})**\n**[Documentation](${products["engine-user-setings"]["doc-url"]})**`;
                thumb_link = `${products["engine-user-setings"]["thumb-link"]}`;
                break;
            case "Internet Protocol":
                description = products["internet-protocol"]["product-desc"]
                links = `**[Buy it on fab](${products["internet-protocol"]["product-url"]})**\n**[Documentation](${products["internet-protocol"]["doc-url"]})**`;
                thumb_link = `${products["internet-protocol"]["thumb-link"]}`;
                break;
        }

        const embed = createEmbed({
            color: "Green",
            title: product,
            thumbnail: thumb_link,
            description: `${description}\n\n${links}`
        });

        interaction.reply({ embeds: [embed] });
    }
})