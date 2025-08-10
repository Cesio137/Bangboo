import { createContainer, createLinkButton, createRow, createSeparator, createTextDisplay } from "@magicyan/discord";
import { ContainerBuilder, MessageActionRowComponentBuilder } from "discord.js";

export function fabComponent(): ContainerBuilder {
    return createContainer({
        accentColor: constants.colors.green,
        components: [
            createTextDisplay("### CHECK OUT MY FAB PRODUCTS", 0),
            createSeparator(false, true),
            createRow<MessageActionRowComponentBuilder>().addComponents(
                [
                    createLinkButton({
                        label: "Engine User Settings",
                        emoji: emojis.static.icons_f,
                        url: fab.engine_user_setings.product_url
                    }),
                    createLinkButton({
                        label: "Documentation",
                        emoji: emojis.static.icons_d,
                        url: fab.engine_user_setings.doc_url
                    })
                ]
            ),
            createTextDisplay(`${fab.engine_user_setings.product_desc}`, 0),
            createSeparator(false, true),
            createRow<MessageActionRowComponentBuilder>().addComponents(
                [
                    createLinkButton({
                        label: "Internet Protocol",
                        emoji: emojis.static.icons_f,
                        url: fab.internet_protocol.product_url
                    }),
                    createLinkButton({
                        label: "Documentation",
                        emoji: emojis.static.icons_d,
                        url: fab.internet_protocol.doc_url
                    })
                ]
            ),
            createTextDisplay(`${fab.internet_protocol.product_desc}`, 0),
        ]
    })
}