import { createModalFields, createLabel, createTextInput, brBuilder } from "@magicyan/discord";
import { ChatInputCommandInteraction, UserSelectMenuBuilder, StringSelectMenuBuilder } from "discord.js";

export async function timeoutModal(interaction: ChatInputCommandInteraction<"cached">) {
    await interaction.showModal({
        title: "Timeout user(s)",
        customId: "modal/moderate/timeout",
        components: createModalFields(
            brBuilder(
                "# Warning",
                "Bangboo will automatically filter and remove the guild owner and moderators if any are selected."
            ),
            createLabel({
                label: "Select user(s)",
                component: new UserSelectMenuBuilder({
                    customId: "users",
                    placeholder: "Select at least one user",
                    minValues: 1,
                    maxValues: 25
                })
            }),
            createLabel({
                label: "Select duration",
                component: new StringSelectMenuBuilder({
                    customId: "duration",
                    placeholder: "Select duration",
                    options: [
                        { label: "60 seconds", value: "60_000", default: true },
                        { label: "05 minutes", value: "300_000" },
                        { label: "10 minutes", value: "600_000" },
                        { label: "01 hour", value: "3600_000" },
                        { label: "01 week", value: "604_800_000" },
                    ],
                    minValues: 1,
                    maxValues: 1,
                })
            }),
            createLabel({
                label: "Reason",
                component: createTextInput({
                    customId: "reason",
                    placeholder: "Visible only in auditlogs channel",
                    maxLength: 500,
                    required: false
                })
            })
        )
    })
}