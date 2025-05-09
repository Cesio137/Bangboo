import { Canvas, loadImage } from "@napi-rs/canvas";
import {
    AttachmentBuilder,
    Events,
    GuildMember,
    PartialGuildMember,
    TextChannel,
    User,
} from "discord.js";
import { join } from "path";

export async function globalMessage( event: Events, member: GuildMember | PartialGuildMember | undefined, user: User, channel: TextChannel) {
    const canvas = new Canvas(1024, 260);
    const context = canvas.getContext("2d");
    const background = await loadImage(
        join(
            __rootname,
            `assets/canvas/${
                event === Events.GuildMemberAdd ? "join" : event === Events.GuildMemberRemove ? "leave" : "ban"
            }.png`
        )
    );
    context.drawImage(background, 0, 0, canvas.width, canvas.height);
    context.save();

    const avatar = await loadImage(user.displayAvatarURL({ size: 256 }));
    context.beginPath();
    context.arc(90 + 68, 90 + 41, 90, 0, Math.PI * 2);
    context.clip();
    context.drawImage(avatar, 68, 41, 180, 180);
    context.restore();

    const actionIconPath = join(
        __rootname,
        `assets/icons/static/${
            event === Events.GuildMemberAdd ? "add" : event === Events.GuildMemberRemove ? "minus" : "hammer"
        }.png`
    );
    const actionIcon = await loadImage(actionIconPath);
    context.drawImage(actionIcon, 205, 179);

    if (event === Events.GuildMemberAdd && typeof member !== "undefined") {
        const { joinedTimestamp } = member;
        const accountAge = Date.now() - user.createdTimestamp;
        const joinAge = Date.now() - (joinedTimestamp || 0);
        const timeLimit = 60 * 1000;
        const welcomeText =
            joinAge < timeLimit && accountAge > timeLimit
                ? "FIRST TIME"
                : "WELCOME BACK";
        context.fillStyle = "#FFFFFF";
        context.font = "semibold 16px Poppins";
        context.textBaseline = "middle";
        const xPos = welcomeText === "FIRST TIME" ? 556 : 533;
        context.fillText(welcomeText, xPos, 66 + 8);
    }

    const { username, displayName } = user;

    let usernameFontSize = 60;
    context.fillStyle = "#FFFFFF";
    do {
        context.font = `bold ${--usernameFontSize}px Rubik`;
    } while (context.measureText(username).width > canvas.width - 400);
    context.textBaseline = "middle";
    context.fillText(username, 300, 110 + usernameFontSize / 2);

    let displayNameFontSize = 32;
    context.fillStyle = "#FFFFFF";
    do {
        context.font = `regular ${--displayNameFontSize}px Lato`;
    } while (context.measureText(`@${displayName}`).width > canvas.width - 400);
    context.textBaseline = "middle";
    context.fillText(`@${displayName}`, 300, 170 + displayNameFontSize / 2);

    const buffer = await canvas.encode("png");
    const attachment = new AttachmentBuilder(buffer, { name: "card.png" });

    let utc  = "";
    if (typeof member !== "undefined" && Events.GuildMemberAdd) {
        const { joinedTimestamp } = member;
        if (joinedTimestamp) {
            utc = `<t:${joinedTimestamp}:F>`;
        }
    } else {
        utc = `<t:${Math.floor(Date.now() / 1000)}:F>`
    }
    
    channel.send({ content: utc, files: [attachment] });
}