import { res } from "#functions";
import { kickMenu, loadMenu, closeMenu } from "#menus";
import { createEmbed } from "@magicyan/discord";
import { User, Guild, InteractionReplyOptions, ChatInputCommandInteraction, ComponentType } from "discord.js";
import { filterUsers, showModal } from "./index.js";


function kickAction<R>(user: User, ids: string[], reason: string, guild: Guild): R {
    let sucess: string[] = [];
    let failed: string[] = [];

    for (const id of ids) {
        const member = guild.members.cache.get(id);
        if (!member) {
            failed.push(id);
            continue;
        }
        member.kick(reason);
        sucess.push(id);
    }

    let description = `**Kicked users:**\n${sucess.map(id => `<@${id}>`).join("\n")}`;
    if (failed.length > 0) {
        description = `${description}\n`
        description = `**Failed to kick user(s):**\n${failed.map(id => `<@${id}>`).join("\n")}`
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

export async function kickCollector(interaction: ChatInputCommandInteraction<"cached">) {
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
    interaction.editReply(kickMenu(user, ids));

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
                await i.editReply(kickMenu(user, ids));
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
                const res = await showModal(i, time);
                if (res.isOk) {
                    i.editReply(kickAction(user, ids, res.reason, guild));
                    timeout = false;
                    isOk = true;
                    userCollector.stop(); btnCollector.stop();
                }
                break;
        }
    });

    btnCollector.on("end", async function() {
        if (!isOk) interaction.editReply(closeMenu(interaction.user, timeout))
    });
}