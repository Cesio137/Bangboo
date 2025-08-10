import { createContainer, createLinkButton, createRow, createSeparator, createTextDisplay } from "@magicyan/discord";
import { ContainerBuilder, MessageActionRowComponentBuilder } from "discord.js";

export function socialComponent(): ContainerBuilder {
    return createContainer({
        accentColor: constants.colors.green,
        components: [
            createTextDisplay("**FOLLOW ME ON SOCIAL MEDIA**", 0),
            createSeparator(false, false),
            createRow<MessageActionRowComponentBuilder>().addComponents(
                [
                    createLinkButton({
                        label: "Portifolio",
                        emoji: emojis.static.icons_p,
                        url: "https://nathan-miguel.vercel.app/"
                    }),
                    createLinkButton({
                        label: "Youtube",
                        emoji: emojis.static.icons_youtube,
                        url: "https://www.youtube.com/@NathanMiguel1"
                    }),
                    createLinkButton({
                        label: "Instagram",
                        emoji: emojis.static.icons_instagram,
                        url: "https://www.instagram.com/nathan_cmiguel/"
                    }),
                    createLinkButton({
                        label: "Github",
                        emoji: emojis.static.icons_github,
                        url: "https://github.com/Cesio137"
                    }),
                    createLinkButton({
                        label: "X/Twitter",
                        emoji: emojis.static.icons_x,
                        url: "https://x.com/NathanCmig"
                    }),
                ]
            ),
            createSeparator(true, true),
            createTextDisplay("**VISIT MY FAB STORE**", 1),
            createSeparator(false, false),
            createLinkButton({
                label: "Fab",
                emoji: emojis.static.icons_f,
                url: "https://www.fab.com/sellers/Nathan%20Miguel"
            }),
        ]
    })
}