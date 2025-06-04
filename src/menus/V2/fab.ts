import { icon, products } from "#functions";
import { settings } from "#settings";
import { createContainer, createLinkButton, createRow, createSeparator, createTextDisplay } from "@magicyan/discord";
import { ContainerBuilder, MessageActionRowComponentBuilder } from "discord.js";

export function fabComponent(): ContainerBuilder {
    return createContainer({
        accentColor: settings.colors.green,
        components: [
            createTextDisplay("### CHECK OUT MY FAB PRODUCTS", 0),
            createSeparator(false, true),
            createRow<MessageActionRowComponentBuilder>().addComponents(
                [
                    createLinkButton({
                        label: "Engine User Settings",
                        emoji: icon.icons_f,
                        url: products["engine-user-setings"]["product-url"]
                    }),
                    createLinkButton({
                        label: "Documentation",
                        emoji: icon.icons_d,
                        url: products["engine-user-setings"]["doc-url"]
                    })
                ]
            ),
            createTextDisplay(`${products["engine-user-setings"]["product-desc"]}`, 0),
            createSeparator(false, true),
            createRow<MessageActionRowComponentBuilder>().addComponents(
                [
                    createLinkButton({
                        label: "Internet Protocol",
                        emoji: icon.icons_f,
                        url: products["internet-protocol"]["product-url"]
                    }),
                    createLinkButton({
                        label: "Documentation",
                        emoji: icon.icons_d,
                        url: products["internet-protocol"]["doc-url"]
                    })
                ]
            ),
            createTextDisplay(`${products["internet-protocol"]["product-desc"]}`, 0),
        ]
    })
}