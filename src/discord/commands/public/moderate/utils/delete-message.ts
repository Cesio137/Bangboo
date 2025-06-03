import { res } from "#functions";
import { deleteMessageMenu, loadMenu, closeMenu } from "#menus";
import { createEmbed } from "@magicyan/discord";
import { User, GuildTextBasedChannel, Guild, GuildMember, InteractionReplyOptions, ChatInputCommandInteraction, ComponentType } from "discord.js";
import { filterUsers } from "./index.js";


async function deleteMessageAction<R>(user: User, ids: string[], channel: GuildTextBasedChannel, guild: Guild): Promise<R> {
    let message_owners: GuildMember[] = [];
    let failed: string[] = [];

    let messages = await channel.messages.fetch({ limit: 100 });
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

export async function deleteMessageCollector(interaction: ChatInputCommandInteraction<"cached">) {
    const { channel, guild, user } = interaction;
    if (!channel) {
        interaction.reply(
            res.danger("Invalid channel.")
        )
        return 
    }

    let ids: string[] = [];
    let timeout = true;
    let isOk = false;

    await interaction.deferReply({ flags: ["Ephemeral"] });
    let messageID = (await interaction.fetchReply()).id;
    interaction.editReply(deleteMessageMenu(user, ids));

    const time = Date.now() + 300000;

    const userCollector = channel.createMessageComponentCollector(
        {
            componentType: ComponentType.UserSelect,
            filter: (componentInteraction) => componentInteraction.user.id === interaction.user.id && componentInteraction.message.id === messageID,
            time: time - Date.now(),
        }
    );

    const btnCollector = channel.createMessageComponentCollector(
        {
            componentType: ComponentType.Button,
            filter: (componentInteraction) => componentInteraction.user.id === interaction.user.id && componentInteraction.message.id === messageID,
            time: time - Date.now(),
        }
    );
    
    userCollector.on("collect", async function(i) {
        const { customId, guild, user } = i;
        if (!guild) { return }
        switch(customId) {
            case "mod/select-users":
                await i.update(loadMenu(user, "👥 **Filtering selected users...**"));
                ids = filterUsers(i.values, guild);
                await i.editReply(deleteMessageMenu(user, ids));
                break;
        }
    });
    
    btnCollector.on("collect", async function(i) {
        const { customId, user } = i;
        switch(customId) {
            case "mod/btn-cancel":
                timeout = false;
                userCollector.stop(); btnCollector.stop();
                break;

            case "mod/btn-confirm":
                await i.update(loadMenu(user, "👥 **Deleting messages from selected users...**"))
                await i.editReply(await deleteMessageAction(user, ids, channel, guild));
                timeout = false;
                isOk = true;
                userCollector.stop(); btnCollector.stop();
                break;
        }
    });

    btnCollector.on("end", async function() {
        if (!isOk) interaction.editReply(closeMenu(interaction.user, timeout));
    });
}