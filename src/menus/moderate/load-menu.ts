import { createEmbed } from "@magicyan/discord";
import { User, InteractionReplyOptions } from "discord.js";

export function loadMenu<R>(user: User, description: string): R {
    const embed = createEmbed({
        color: "Blue",
        author: {
            name: user.globalName || user.username,
            iconURL: user.avatarURL() || undefined
        },
        title: "**Officer Cui's panel**",
        thumbnail: "https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/avatar/Officer.png",
        description
    });

    return ({
            flags: ["Ephemeral"],
            embeds: [embed]
        } satisfies InteractionReplyOptions) as R;
}