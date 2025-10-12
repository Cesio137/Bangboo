import { createResponder, ResponderType } from "#base";
import { logger, res } from "#functions";
import { createEmbed, modalFieldsToRecord } from "@magicyan/discord";
import { User, Guild, InteractionReplyOptions, GuildMember, GuildTextBasedChannel } from "discord.js";
import z from "zod";

const schema = z.object({
    users: z.array(z.string(), "Expected array of users to timeout"),
    limit: z.string().transform(Number),
});

createResponder({
    customId: "modal/moderate/delete_message",
    types: [ResponderType.Modal],
    cache: "cached",
    async run(interaction) {
        const modalFields = schema.safeParse(modalFieldsToRecord(interaction.fields));
        if (!modalFields.success) {
            await interaction.reply(res.danger("Failed to parse timeout modal fields", { flags: ["Ephemeral"] }));
            logger.error(`Failed to parse delete-message modal fields\n${modalFields.error}`);
            return;
        }

        await interaction.deferReply({ flags: ["Ephemeral"] });

        const { guild, channel, user } = interaction;
        if (!guild) {
            await interaction.reply(res.danger("Failed to get guild", { flags: ["Ephemeral"] }));
            return;
        }

        if (!channel) {
            await interaction.reply(res.danger("Failed to get guild channel", { flags: ["Ephemeral"] }));
            return;
        }

        await interaction.editReply(await deleteMessageAction(user, modalFields.data.users, modalFields.data.limit, channel as GuildTextBasedChannel, guild));
    },
});

async function deleteMessageAction<R>(user: User, ids: string[], limit: number, channel: GuildTextBasedChannel, guild: Guild): Promise<R> {
    let message_owners: GuildMember[] = [];
    let failed: string[] = [];

    let messages = await channel.messages.fetch({ limit });
    for (const id of ids) {
        const member = guild.members.cache.get(id);
        if (!member) {
            failed.push(id);
            continue;
        }
        message_owners.push(member);
    }

    if (message_owners.length == 0) {
        const embed = createEmbed({
            color: "Blue",
            author: {
                name: user.globalName || user.username,
                iconURL: user.avatarURL() || undefined
            },
            title: "**Officer Cui's panel**",
            thumbnail: "https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/avatar/Officer.png",
            description: "**Failed to delete message(s): No user(s) selected**"
        });

        return ({
            flags: ["Ephemeral"],
            embeds: [embed],
            components: [],
        } satisfies InteractionReplyOptions) as R;
    }

    messages = messages.filter(msg => msg.member && message_owners.indexOf(msg.member) != -1);
    if (messages.size > 0) await channel.bulkDelete(messages, true);

    let description = `**Deleted message(s) from users:**\n${message_owners.map(id => `<${id}>`).join("\n")}`;
    if (failed.length > 0) {
        description = `${description}\n`
        description = `**Failed to delete message(s) from user(s):**\n${failed.map(id => `<${id}>`).join("\n")}`
    }

    const embed = createEmbed({
        color: "Blue",
        author: {
            name: user.globalName || user.username,
            iconURL: user.avatarURL() || undefined
        },
        title: "**Officer Cui's panel**",
        thumbnail: "https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/avatar/Officer.png",
        description: description
    });

    return ({
        flags: ["Ephemeral"],
        embeds: [embed],
        components: [],
    } satisfies InteractionReplyOptions) as R;
}