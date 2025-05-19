import { createEmbed } from "@magicyan/discord";
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
import { channels, icon, roles } from "#functions";
import { settings } from "#settings";

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
        `assets/icons/static/Card/${
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
    if (event === Events.GuildMemberAdd && typeof member !== "undefined") {
        const { joinedTimestamp } = member;
        if (joinedTimestamp) {
            const timestamp = Math.floor(joinedTimestamp / 1000);
            utc = `<t:${timestamp}:F>`;
        }
    } else {
        const timestamp = Math.floor(Date.now() / 1000);
        utc = `<t:${timestamp}:F>`
    }
    
    channel.send({ content: utc, files: [attachment] });
}

export async function globalBoost(member: GuildMember) {
    const { guild, id, user } = member;
    const { globalName, username } = user;
    const guild_channel = await guild.channels.fetch(channels.announcement);
    if (!guild_channel) return;
    const avatarURL = user.avatarURL({size: 256});
    const embed = createEmbed({
        color: settings.colors.nitro,
        author: {
            name: globalName || username,
            iconURL: avatarURL || undefined
        },
        description: `**${icon.boost} <@${id}> became a <@&${roles.boosters}>**\n\n🚀 Thanks for boosting the server!`,
        thumbnail: avatarURL || undefined
    });
    const channel = await guild_channel.fetch();
    if (channel.isTextBased()) {
        channel.send({content: "||@everyone @here||", embeds: [embed]});
    }
}