import { logger } from "#functions";
import { createEmbed } from "@magicyan/discord";
import { Message, OmitPartialGroupDMChannel } from "discord.js";

const regex = new RegExp("\\[\\s*steam[^\\]]*]\\((https?:\\/\\/[^)]+)\\)", "gi");

export async function filterMessage(
    message: OmitPartialGroupDMChannel<Message<boolean>>
) {
    const { author, content, guild, member } = message;
    if (!regex.test(content)) return;

    const username = author.globalName || author.username;
    let warning_message = `**${username}** sent a message that was flagged as a scam. Messages containing ***[text](hyperlink)*** are strictly prohibited. Bangboo (me) will presume his/her account has been compromised, leading to a server kick!`;
    const embed = createEmbed({
        color: "Yellow",
        description: warning_message,
    });
    await message.reply({ embeds: [embed] });
    await message.delete();

    if (!guild) {
        logger.error("Failed to fetch guild.");
        return;
    }

    if (guild.ownerId === author.id) {
        logger.error("Tried to kick the owner of the guild");
        return;
    }

    if (!member) {
        logger.error("Failed to fetch member.");
        return;
    }

    member.kick(`${username} sent a scam message!`);
    const embed_dm = createEmbed({
        color: "Yellow",
        description:
            "It look like you probably got hacked and sent a message that was flagged as scam containing ***[text](hyperlink)***. You were just kicked from the server, but feel free to come back as soon as you resolve the issue with your account.",
    });
    author.send({ embeds: [embed_dm] });
}
