import { createModalFields, createLabel, createTextInput } from "@magicyan/discord";
import { ChatInputCommandInteraction, UserSelectMenuBuilder } from "discord.js";

export async function deleteMessageModal(interaction: ChatInputCommandInteraction<"cached">) {
    await interaction.showModal({
        title: "Delete message",
        customId: "modal/moderate/delete_message",
        components: createModalFields(
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
                label: "Limit",
                description: "Limit of messages to fetch",
                component: createTextInput({
                    customId: "limit",
                    minLength: 1,
                    maxLength: 4,
                    placeholder: "Numbers from 1 to 1000"
                })
            })
        )
    })
}