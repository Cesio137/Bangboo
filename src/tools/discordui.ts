import { icon } from "#functions";
import { discordUI } from "@magicyan/discord-ui";

discordUI({
    menus: {
        multimenu: {
            buttons: {
                previous: {
                    emoji: icon.back,
                    label: "",
                },
                home: {
                    emoji: icon.home,
                    label: "",
                },
                next: {
                    emoji: icon.next,
                    label: "",
                },
            },
        },
    },
});
