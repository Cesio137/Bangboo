import { createCommand } from "#base";
import { res, roles } from "#functions";
import { banMenu, closeMenu, deleteMessageMenu, kickMenu, loadMenu, timeoutMenu } from "#menus";
import { createEmbed, createModalFields, modalFieldsToRecord } from "@magicyan/discord";
import { ApplicationCommandOptionType, ApplicationCommandType, ButtonInteraction, CacheType, ChatInputCommandInteraction, ComponentType, Guild, GuildMember, GuildTextBasedChannel, InteractionContextType, InteractionReplyOptions, User } from "discord.js";

createCommand({
    name: "moderate",
    description: "Equality before the law is the cornerstone of justice ⚖.",
    type: ApplicationCommandType.ChatInput,
    contexts: [InteractionContextType.Guild],
    options: [
        {
            name: "action",
            description: "command option",
            required: true,
            type: ApplicationCommandOptionType.String,
            choices: [ 
                { name: "delete messages", value: "delete_messages" },
                { name: "timeout", value: "timeout" },
                { name: "kick", value: "kick" },
                { name: "ban", value: "ban" },
            ]
        }
    ],
    async run(interaction) {
        const { member, options } = interaction;
        if (!member.permissions.has("Administrator")) {
            interaction.reply(
                res.danger("You don't have **ADMINISTRATOR** permission.")
            )
        }
        
        switch(options.getString("action", true)) {
            case "delete_messages":
                deleteMessageCollector(interaction);
                break
            case "timeout":
                timeoutCollector(interaction);
                break
            case "kick":
                kickCollector(interaction);
                break
            case "ban":
                banCollector(interaction);
                break
        }
    }
});

function filterUsers(ids: string[], guild: Guild): string[] {
    let users: string[] = [];
    if (ids.length < 1) { return users; }
    for (const id of ids) {
        const member = guild.members.cache.get(id);
        if (!member) {continue}
        if (!member.user.bot && !member.permissions.has("Administrator") && !member.roles.cache.has(roles.stf)) {
            users.push(member.id);
        }
    }

    return users;
}

async function showModal(interaction: ButtonInteraction<CacheType>, time: number): Promise<{isOk: boolean, reason: string}> {
    let isOk = false;
    let reason = "";
    await interaction.showModal({
        custom_id: "mod/modal-reason",
        title: "What's the reason?",
        components: createModalFields({
            reason: {
                label: "Reason",
                placeholder: "Visible only in auditlogs",
                maxLength: 300,
                minLength: 0,
            }
        }).map(component => component.toJSON())
    });
    await interaction.awaitModalSubmit({ time: time - Date.now() })
        .then(async modalInteraction => {
            await modalInteraction.deferUpdate();
            reason = modalFieldsToRecord(modalInteraction.fields).reason;
            isOk = true;
        })
        .catch(() => {
            interaction.followUp(res.danger("Modal submission timed out."));
        })
    return {isOk, reason};
}

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

async function deleteMessageCollector(interaction: ChatInputCommandInteraction<"cached">) {
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

// Timeout
function timeoutAction<R>(user: User, ids: string[], duration: string, reason: string, guild: Guild): R {
    let sucess: string[] = [];
    let failed: string[] = [];

    for (const id of ids) {
        const member = guild.members.cache.get(id);
        if (!member) {
            failed.push(id);
            continue;
        }
        member.timeout(parseInt(duration) * 1000, reason);
        sucess.push(id);
    }

    let description = `**Timeouted users:**\n${sucess.map(id => `<@${id}>`).join("\n")}`;
    if (failed.length > 0) {
        description = `${description}\n`
        description = `**Failed to timeout user(s):**\n${failed.map(id => `<@${id}>`).join("\n")}`
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

async function timeoutCollector(interaction: ChatInputCommandInteraction<"cached">) {
    const { channel, guild, user } = interaction;
    if (!channel) {
        interaction.reply(
            res.danger("Invalid channel.")
        )
        return 
    }

    let ids: string[] = [];
    let duration: string = "";
    let timeout = true;
    let isOk = false;

    await interaction.deferReply({ flags: ["Ephemeral"] });
    let messageID = (await interaction.fetchReply()).id;
    interaction.editReply(timeoutMenu(user, ids, duration));

    const time = Date.now() + 300000;

    const userCollector = channel.createMessageComponentCollector(
        {
            componentType: ComponentType.UserSelect,
            filter: (componentInteraction) => componentInteraction.user.id === interaction.user.id && componentInteraction.message.id === messageID,
            time: time - Date.now(),
        }
    );

    const durationCollector = channel.createMessageComponentCollector(
        {
            componentType: ComponentType.StringSelect,
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
                await i.editReply(timeoutMenu(user, ids, duration));
                break;
        }
    });

    durationCollector.on("collect", async function(i) {
        const { customId, user } = i;
        switch(customId) {
            case "mod/select-duration":
                duration = i.values[0];
                await i.update(timeoutMenu(user, ids, duration))
                break;
        }
    });
    
    btnCollector.on("collect", async function(i) {
        const { customId, user } = i;
        switch(customId) {
            case "mod/btn-cancel":
                timeout = false;
                userCollector.stop(); durationCollector.stop(); btnCollector.stop();
                break;

            case "mod/btn-confirm":
                const res = await showModal(i, time);
                if (res.isOk) {
                    i.editReply(timeoutAction(user, ids, duration, res.reason, guild));
                    timeout = false;
                    isOk = true;
                    userCollector.stop(); durationCollector.stop(); btnCollector.stop();
                }
                break;
        }
    });

    btnCollector.on("end", async function() {
        if (!isOk) interaction.editReply(closeMenu(interaction.user, timeout));
    });
}

// Kick
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

async function kickCollector(interaction: ChatInputCommandInteraction<"cached">) {
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

// Ban

function banAction<R>(user: User, ids: string[], reason: string, guild: Guild): R {
    let sucess: string[] = [];
    let failed: string[] = [];

    for (const id of ids) {
        const member = guild.members.cache.get(id);
        if (!member) {
            failed.push(id);
            continue;
        }
        member.ban({reason});
        sucess.push(id);
    }

    let description = `**Banned users:**\n${sucess.map(id => `<@${id}>`).join("\n")}`;
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

async function banCollector(interaction: ChatInputCommandInteraction<"cached">) {
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
    interaction.editReply(banMenu(user, ids));

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
        console.log(i.message.id);
        const { customId, guild, user } = i;
        if (!guild) { return }
        switch(customId) {
            case "mod/select-users":
                await i.update(loadMenu(user, "👥 **Filtering selected users...**"));
                ids = filterUsers(i.values, guild);
                await i.editReply(banMenu(user, ids));
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
                    i.editReply(banAction(user, ids, res.reason, guild));
                    timeout = false;
                    isOk = true;
                    userCollector.stop(); btnCollector.stop();
                }
                break;
        }
    });

    btnCollector.on("end", async function() {
        if (!isOk) interaction.editReply(closeMenu(interaction.user, timeout));
    });
}