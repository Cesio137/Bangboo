import { getPhashFromImageBuffer, logger } from "#functions";
import { createEmbed } from "@magicyan/discord";
import { Message, OmitPartialGroupDMChannel } from "discord.js";
import { closestMatch } from "leven";

export async function filterAttachment(message: OmitPartialGroupDMChannel<Message<boolean>>) {
    const { attachments, author, guild, member } = message;

    if (!guild) return;
    if (attachments.size == 0) return;

    const urls = attachments.map(attachment => {
        const { contentType, size, url, width, height, } = attachment;
        if (contentType &&
            size <= 1_100_000 &&
            contentType.startsWith("image/") &&
            !contentType.startsWith("image/gif")) {
            if (width && height) return url;
        }
        return undefined;
    }).filter(url => (typeof url !== "undefined"));

    if (urls.length == 0) return;

    try {
        for (const url of urls) {
            const res = await fetch(url);
            const buf = await res.arrayBuffer();

            const phash = await getPhashFromImageBuffer(buf);
            const match = closestMatch(phash, hashes.filters, { maxDistance: 1 });
            if (typeof match === "string") {
                let warning_message = `<@${author.id}> sent a image that was identified as a scam. Bangboo (me) will presume his/her account has been compromised, leading to a server kick!`;
                const embed = createEmbed({
                    color: "Yellow",
                    description: warning_message,
                });
                await message.reply({ embeds: [embed] });
                await message.delete();

                if (guild.ownerId === author.id) {
                    logger.error("Tried to kick the owner of the guild");
                    return;
                }

                if (!member) {
                    logger.error("Failed to fetch member.");
                    return;
                }

                await member.kick(`${member.user.username} sent a scam image!`);
                const embed_dm = createEmbed({
                    color: "Yellow",
                    description:
                        "It look like you probably got hacked and sent a scam image. You were just kicked from the server, but feel free to come back as soon as you resolve the issue with your account.",
                });
                await author.send({ embeds: [embed_dm] });
                break;
            }
        }
    } catch (e) {
        logger.error(e);
    }


}

/*
const regexSteam = new RegExp("\\[\\s*steam[^\\]]*]\\((https?:\\/\\/[^)]+)\\)", "gi");
const attachmentPattern = new RegExp("(?:(?:https?:\\/\\/)?(?:cdn|media)\\.discordapp\\.(?:net|com)\\/attachments\\/\\d+\\/\\d+\\/[^\\s]+\\.(?:png|jpg|jpeg|webp)(?:\\?[^\\s]*)?\\s*){3,}", "g");

export async function filterMessage(
    message: OmitPartialGroupDMChannel<Message<boolean>>
) {
    const { author, content, guild, member } = message;

    if (!guild) return;

    const steamTest = regexSteam.test(content);
    const attachmentTest = attachmentPattern.test(content);

    if (!steamTest && !attachmentTest) return;
    
    const username = author.globalName || author.username;
    let warning_message = `<@${author.id}> sent a message that was flagged as a scam. Messages containing **[steam...](hyperlink)** or **more than 2 image attachments** are strictly prohibited. Bangboo (me) will presume his/her account has been compromised, leading to a server kick!`;
    const embed = createEmbed({
        color: "Yellow",
        description: warning_message,
    });
    await message.reply({ embeds: [embed] });
    await message.delete();

    if (guild.ownerId === author.id) {
        logger.error("Tried to kick the owner of the guild");
        return;
    }

    if (!member) {
        logger.error("Failed to fetch member.");
        return;
    }

    await member.kick(`${username} sent a scam message!`);
    const embed_dm = createEmbed({
        color: "Yellow",
        description:
            "It look like you probably got hacked and sent a message that was flagged as scam containing **[steam...](hyperlink)** or **more than 2 image attachments**. You were just kicked from the server, but feel free to come back as soon as you resolve the issue with your account.",
    });
    await author.send({ embeds: [embed_dm] });
}
*/