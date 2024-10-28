import { Canvas, loadImage } from "@napi-rs/canvas";
import {
    AttachmentBuilder,
    ChannelType,
    Events,
    GuildMember,
    PartialGuildMember,
} from "discord.js";
import { join } from "path";

export async function globalMessage(event: Events, member: GuildMember | PartialGuildMember) {
    const canvas = new Canvas(800, 200);
    const context = canvas.getContext("2d");

    const background = await loadImage(
        join(__rootname, `assets/images/${event === Events.GuildMemberAdd ? "join" : "leave"}.png`)
    );
    context.drawImage(background, 0, 0, canvas.width, canvas.height);
    context.save();

    const avatar = await loadImage(member.displayAvatarURL({ size: 256 }));
    context.beginPath();
    context.arc(64 + 32, 64 + 36, 64, 0, Math.PI * 2);
    context.clip();
    context.drawImage(avatar, 32, 36, 128, 128);
    context.restore();

    const actionIconPath = join(
        __rootname,
        `assets/icons/static/${event === Events.GuildMemberAdd ? "add" : "minus"}.svg`
    );
    const actionIcon = await loadImage(actionIconPath);
    context.drawImage(actionIcon, 180, 74 - 16);

    const actionText = event === Events.GuildMemberAdd ? "JOINED TO SERVER" : "LEFT THE SERVER";
    context.fillStyle = "#FFFFFF";
    context.font = "extrabold 24px Montserrat";
    context.textBaseline = "middle";
    context.fillText(actionText, 180, 117);

    const { displayName } = member;
    let fontSize = 48;
    context.fillStyle = "#FFFFFF";
    do {
        context.font = `${--fontSize}px Heavitas`;
    } while (context.measureText(displayName).width > canvas.width - 240);
    context.textBaseline = "middle";
    context.fillText(displayName, 220, 78);

    const buffer = await canvas.encode("png");
    const attachment = new AttachmentBuilder(buffer, { name: "card.png" });

    const { guild } = member;
    const channel = guild.channels.cache.find((ch) => ch.name === "😏┊welcome");
    if (channel?.type !== ChannelType.GuildText) {
        console.log("Channel not found");
        return;
    }
    channel.send({ files: [attachment] });
}
