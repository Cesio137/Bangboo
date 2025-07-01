import { createEvent } from "#base";
import { brBuilder } from "@magicyan/discord";
import { ActivitiesOptions, ActivityType } from "discord.js";

createEvent({
    name: "ready",
    event: "ready",
    async run(client) {
        client.application.edit({
            description: brBuilder(
                "**Bangboo**",
                "",
                "Bangboo created by **Nathan Miguel** to watch his server!",
                "",
                "Source code: https://github.com/Cesio137/Bangboo"
            )
        });

        const status: ActivitiesOptions[] = [
            { name: "Whatching Nathan's server!", type:ActivityType.Custom },
            { name: "Join in: .gg/DBNATxA6Jx", type:ActivityType.Custom },
            { name: "Zenless Zone Zero!", type:ActivityType.Custom },
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
            20000
        );        
    },
});