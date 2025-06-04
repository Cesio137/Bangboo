import { icon } from "#functions";
import { settings } from "#settings";
import { createContainer, createLinkButton, createRow, createSeparator, createTextDisplay } from "@magicyan/discord";
import { ContainerBuilder, MessageActionRowComponentBuilder } from "discord.js";

export function socialComponent(): ContainerBuilder {
    return createContainer({
        accentColor: settings.colors.green,
        components: [
            createTextDisplay("**FOLLOW ME ON SOCIAL MEDIA**", 0),
            createSeparator(false, false),
            createRow<MessageActionRowComponentBuilder>().addComponents(
                [
                    createLinkButton({
                        label: "Youtube",
                        emoji: icon.icons_youtube,
                        url: "https://www.youtube.com/@NathanMiguel1"
                    }),
                    createLinkButton({
                        label: "Instagram",
                        emoji: icon.icons_instagram,
                        url: "https://www.instagram.com/nathan_cmiguel/"
                    }),
                    createLinkButton({
                        label: "Github",
                        emoji: icon.icons_github,
                        url: "https://github.com/Cesio137"
                    }),
                    createLinkButton({
                        label: "Linkedin",
                        emoji: icon.icons_linked,
                        url: "https://www.linkedin.com/in/nathan-miguel-488b462b1/"
                    }),
                ]
            ),
            createSeparator(true, true),
            createTextDisplay("**VISIT MY FAB STORE**", 1),
            createSeparator(false, false),
            createLinkButton({
                label: "Fab",
                emoji: icon.icons_f,
                url: "https://www.fab.com/sellers/Nathan%20Miguel"
            }),
        ]
    })
}