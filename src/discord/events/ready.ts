import { createEvent } from "#base";
import { brBuilder } from "@magicyan/discord";
import { ActivitiesOptions, ActivityType } from "discord.js";

createEvent({
    name: "ready",
    event: "ready",
    async run(client) {
        await client.application.edit({
            description: brBuilder(
                "Bangboo are small, rabbit-like robots. They were invented by the Void Hunter **Sunbringer**, Bangboo have since evolved to become **comprehensive and versatile intelligent individuals** that are indispensable helpers in New Eridu's daily operations. They are commonly seen in all parts of New Eridu.",
                "",
                "Source code: https://github.com/Cesio137/Bangboo"
            )
        });

        const status: ActivitiesOptions[] = [
            { name: "Whatching Nathan's server!", type:ActivityType.Custom },
            { name: "Join in: .gg/DBNATxA6Jx", type:ActivityType.Custom },
            { name: "Zenless Zone Zero!", type:ActivityType.Custom },
            { name: "I speak in grunts (textualized as **Ehn-ne's**)", type:ActivityType.Custom },
        ];

        let i = 0;
        setInterval(
            function() {
                client.user.setPresence({
                    status: "online",
                    activities: [status[i % status.length]]
                });
                i++;
            },
            30_000
        );        
    },
});