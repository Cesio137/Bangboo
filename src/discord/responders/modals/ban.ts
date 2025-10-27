import { createResponder, ResponderType } from "#base";
import { filterUsers, logger, res } from "#functions";
import { createEmbed, modalFieldsToRecord } from "@magicyan/discord";
import { User, Guild, InteractionReplyOptions } from "discord.js";
import z from "zod";

const schema = z.object({
    users: z.array(z.string(), "Expected array of users to timeout"),
    reason: z.string().optional(),
});

createResponder({
    customId: "modal/moderate/ban",
    types: [ResponderType.Modal],
    cache: "cached",
    async run(interaction) {
        const modalFields = schema.safeParse(modalFieldsToRecord(interaction.fields));
        if (!modalFields.success) {
            await interaction.reply(res.danger("Failed to parse timeout modal fields", { flags: ["Ephemeral"] }));
            logger.error(`Failed to parse ban modal fields\n${modalFields.error}`);
            return;
        }

        await interaction.deferReply({ flags: ["Ephemeral"] });

        const { guild, user } = interaction;
        if (!guild) {
            await interaction.reply(res.danger("Failed to get guild", { flags: ["Ephemeral"] }));
            return;
        }

        const filteredIds = filterUsers(modalFields.data.users, guild);
        await interaction.editReply(await banAction(user, filteredIds, modalFields.data.reason, guild));
    },
});

async function banAction<R>(user: User, ids: string[], reason: string | undefined, guild: Guild): Promise<R> {
    let success: string[] = [];
    let failed: string[] = [];

    const bulkBanRes = await guild.bans.bulkCreate(ids, { reason, deleteMessageSeconds: 2_592_000 });
    success = bulkBanRes.bannedUsers.values().toArray();
    failed = bulkBanRes.failedUsers.values().toArray();

    let description = `**Banned users:**\n${success.map(id => `<@${id}>`).join("\n")}`;
    if (failed.length > 0) {
        description = `${description}\n`
        description = `**Failed to ban user(s):**\n${failed.map(id => `<@${id}>`).join("\n")}`
    }

    const embed = createEmbed({
        color: "Blue",
        author: {
            name: user.globalName || user.username,
            iconURL: user.avatarURL() || undefined
        },
        title: "**Officer Cui's panel**",
        thumbnail: "https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/avatar/Officer.png",
        description: description,
        footer: `Reason: ${reason}`
    });

    return ({
        flags: ["Ephemeral"],
        embeds: [embed],
        components: [],
    } satisfies InteractionReplyOptions) as R;
}