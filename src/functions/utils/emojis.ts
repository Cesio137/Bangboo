import { settings } from "#settings";
import { formatEmoji } from "discord.js";

type EmojiList = typeof settings.emojis;
type EmojiKey = keyof EmojiList["animated"] | keyof EmojiList["static"];
type IconInfo = { id: string; animated: boolean; toString(): string };
type Icon = Record<EmojiKey, IconInfo>;

const icon: Icon = Object.create({});
formatEmoji("sda", false);
for (const [name, id] of Object.entries(settings.emojis.static)) {
    const data = {
        id,
        animated: false,
        toString() {
            return formatEmoji(id, false);
        },
    };
    Object.assign(icon, { [name]: data });
}
/*for (const [name, id] of Object.entries(settings.emojis.animated)) {
    const data = {
        id,
        animated: false,
        toString() {
            return formatEmoji(id, false);
        },
    };
    Object.assign(icon, { [name]: data });
}*/

export { icon };
