import { createModalFields, createLabel, createTextInput, brBuilder } from "@magicyan/discord";
import { ChatInputCommandInteraction, UserSelectMenuBuilder } from "discord.js";

export async function banModal(interaction: ChatInputCommandInteraction<"cached">) {
    await interaction.showModal({
        title: "Ban user(s)",
        customId: "modal/moderate/ban",
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
                label: "Reason",
                component: createTextInput({
                    customId: "reason",
                    placeholder: "Visible in auditlogs channel",
                    maxLength: 500,
                    required: false
                })
            })
        )
    })
}