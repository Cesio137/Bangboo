import { Canvas } from "@napi-rs/canvas";
import { ApplicationCommandType, AttachmentBuilder } from "discord.js";
import { Command } from "#base";

new Command({
    name: "canvas",
    description: "Canvas command.",
    type: ApplicationCommandType.ChatInput,
    async run(interaction){
        await interaction.deferReply();

        const canvas = new Canvas(600, 300);
        const context = canvas.getContext("2d");

        const colors = {
            background: "#64A9E9",
        };

        context.fillStyle = colors.background;
        context.fillRect(0, 0, canvas.width, canvas.height);

        const buffer = await canvas.encode("png");
        const attachment = new AttachmentBuilder(buffer, { name: "image.png" });

        interaction.editReply({ files: [ attachment ] });
    }
});