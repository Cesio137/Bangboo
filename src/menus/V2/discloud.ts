import { settings } from "#settings";
import { createContainer, createTextDisplay, createSeparator, createRow } from "@magicyan/discord";
import { ButtonBuilder, ButtonStyle, ContainerBuilder, MessageActionRowComponentBuilder } from "discord.js";

export function statusComponent(infos: string[]): ContainerBuilder {
    return createContainer({
        accentColor: settings.colors.green,
        components: [
            createTextDisplay("### BANGBOO'S STATUS", 0),
            createSeparator(false, true),
            createTextDisplay(`${infos.join('\n')}`, 0),
            createSeparator(false, true),
            createRow<MessageActionRowComponentBuilder>().addComponents(
                [
                    new ButtonBuilder()
                        .setStyle(ButtonStyle.Success)
                        .setLabel("Refresh")
                        .setCustomId("discloud/status/refresh")
                ]
            ),
        ]
    })
}

export function logsComponent(logs: string): ContainerBuilder {
    return createContainer({
        accentColor: settings.colors.green,
        components: [
            createTextDisplay("### BANGBOO'S LOGS", 0),
            createSeparator(false, true),
            createTextDisplay(`\`\`\`bash\n${logs}\n\`\`\``, 0),
            createSeparator(false, true),
            createRow<MessageActionRowComponentBuilder>().addComponents(
                [
                    new ButtonBuilder()
                        .setStyle(ButtonStyle.Success)
                        .setLabel("Refresh")
                        .setCustomId("discloud/logs/refresh")
                ]
            ),
        ]
    })
}