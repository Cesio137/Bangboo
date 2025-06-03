import { createEmbed, createRow } from "@magicyan/discord";
import { User, UserSelectMenuBuilder, StringSelectMenuBuilder, ButtonBuilder, ButtonStyle, InteractionReplyOptions } from "discord.js";

export function timeoutMenu<R>(user: User, ids: string[], duration: string): R {
    const embed = createEmbed({
        color: "Blue",
        author: {
            name: user.globalName || user.username,
            iconURL: user.avatarURL() || undefined
        },
        title: "**Officer Cui's panel**",
        thumbnail: "https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/avatar/Officer.png",
        description: "🖱️ **Select user(s) and timeout duration!**"
    });

    const userRow = createRow<UserSelectMenuBuilder>().addComponents(
        new UserSelectMenuBuilder()
            .setCustomId(`mod/select-users`)
            .setPlaceholder("Select user(s)")
            .setDefaultUsers(ids)
            .setMinValues(0)
            .setMaxValues(25)
    );

    const durationRow =  createRow<StringSelectMenuBuilder>().addComponents(
        new StringSelectMenuBuilder()
            .setCustomId(`mod/select-duration`)
            .setPlaceholder("Select duration")
            .addOptions([
                { label: "60 seconds", value: "60_000", default: duration === "60_000" },
                { label: "05 minutes", value: "300_000", default: duration === "300_000" },
                { label: "10 minutes", value: "600_000", default: duration === "600_000" },
                { label: "01 hour", value: "3600_000", default: duration === "3600_000" },
                { label: "01 week", value: "604_800_000", default: duration === "604_800_000" },
            ])
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
            .setDisabled(ids.length < 1 || duration === "")
    );

    return ({
            flags: ["Ephemeral"],
            components: [userRow, durationRow, confirmRow],
            embeds: [embed]
        } satisfies InteractionReplyOptions) as R;
}