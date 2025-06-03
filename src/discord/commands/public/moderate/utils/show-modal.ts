import { res } from "#functions";
import { createModalFields, modalFieldsToRecord } from "@magicyan/discord";
import { ButtonInteraction, CacheType } from "discord.js";


export async function showModal(interaction: ButtonInteraction<CacheType>, time: number): Promise<{isOk: boolean, reason: string}> {
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