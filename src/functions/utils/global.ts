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
    const canvas = new Canvas(2800, 560);
    const context = canvas.getContext("2d");
    
    let bgpath = join(__rootname, "assets/canvas/");
    if (event === Events.GuildMemberAdd && typeof member !== "undefined") {
        const { joinedTimestamp } = member;
        const accountAge = Date.now() - user.createdTimestamp;
        const joinAge = Date.now() - (joinedTimestamp || 0);
        const timeLimit = 60 * 1000;
        bgpath =
            joinAge < timeLimit && accountAge > timeLimit
                ? join(bgpath, "card-new-member.png")
                : join(bgpath, "card-back-to-server.png");
    } else {
        bgpath = join(bgpath, event === Events.GuildMemberRemove ? "card-left.png" : "card-mod.png");
    }
    const background = await loadImage(bgpath);
    context.drawImage(background, 0, 0, canvas.width, canvas.height);
    context.save();

    const avatar = await loadImage(user.displayAvatarURL({ size: 512 }));
    context.beginPath();
    context.arc(200, 200 + 160, 200, 0, Math.PI * 2);
    context.clip();
    context.drawImage(avatar, 0, 160, 400, 400);
    context.restore();

    const { username, displayName } = user;

    let usernameFontSize = 200;
    context.fillStyle = "#FFFFFF";
    do {
        context.font = `medium ${--usernameFontSize}px Fredoka`;
    } while (context.measureText(username).width > canvas.width - 400);
    context.textBaseline = "middle";
    context.fillText(username, 530, 140 + usernameFontSize / 2);

    let displayNameFontSize = 96;
    context.fillStyle = "#FFFFFF";
    do {
        context.font = `medium ${--displayNameFontSize}px Roboto`;
    } while (context.measureText(`@${displayName}`).width > canvas.width - 400);
    context.textBaseline = "middle";
    context.fillText(`@${displayName}`, 530, 380 + displayNameFontSize / 2);

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