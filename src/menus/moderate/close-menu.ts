import { createEmbed } from "@magicyan/discord";
import { User, InteractionReplyOptions } from "discord.js";

export function closeMenu<R>(user: User, timeout: boolean): R {
    const embed = createEmbed({
        color: "Blue",
        author: {
            name: user.globalName || user.username,
            iconURL: user.avatarURL() || undefined
        },
        title: "**Officer Cui's panel**",
        thumbnail: "https://raw.githubusercontent.com/Cesio137/Bangboo/refs/heads/master/assets/avatar/Officer.png",
        description: timeout ? "⏰ **Timeout!**" : "👋 **Bye!**"
    });

    return ({
            flags: ["Ephemeral"],
            components: [],
            embeds: [embed]
        } satisfies InteractionReplyOptions) as R;
}