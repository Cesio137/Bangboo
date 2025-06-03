import { createEmbed, createRow } from "@magicyan/discord";
import { User, UserSelectMenuBuilder, ButtonBuilder, ButtonStyle, InteractionReplyOptions } from "discord.js";

export function kickMenu<R>(user: User, ids: string[]): R {
    const embed = createEmbed({
        color: "Blue",
        author: {
            name: user.globalName || user.username,
            iconURL: user.avatarURL() || undefined
        },
        title: "**Officer Cui's panel**",
        thumbnail: "https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/avatar/Officer.png",
        description: "🖱️ **Select user(s) to kick!**"
    });

    const userRow = createRow<UserSelectMenuBuilder>().addComponents(
        new UserSelectMenuBuilder()
            .setCustomId(`mod/select-users`)
            .setPlaceholder("Select user(s)")
            .setDefaultUsers(ids)
            .setMinValues(0)
            .setMaxValues(25)
    );

    const confirmRow = createRow<ButtonBuilder>().addComponents(
        new ButtonBuilder()
            .setCustomId("mod/btn-cancel")
            .setLabel("Cancel")
            .setStyle(ButtonStyle.Danger),
        new ButtonBuilder()
            .setCustomId("mod/btn-confirm")
            .setLabel("Confirm")
            .setStyle(ButtonStyle.Success)
            .setDisabled(ids.length < 1)
    );

    return ({
            flags: ["Ephemeral"],
            components: [userRow, confirmRow],
            embeds: [embed]
        } satisfies InteractionReplyOptions) as R;
}