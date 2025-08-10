import constantsJson from "../../../constants.json" with { type: "json" };
import emojisJson from "../../../data/emojis.json" with { type: "json" };
import fabJson from "../../../data/fab.json" with { type: "json" };
import guildJson from "../../../data/guild.json" with { type: "json" };

declare global {
    const constants: typeof constantsJson;
    const emojis: typeof emojisJson;
    const guildData: typeof guildJson;
    const fab: typeof fabJson;
}
Object.assign(globalThis, Object.freeze({
    constants: constantsJson,
    emojis: emojisJson,
    guildData: guildJson,
    fab: fabJson,
}));
