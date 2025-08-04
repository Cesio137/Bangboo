import { createEvent } from "#base";
import { ActivitiesOptions, ActivityType } from "discord.js";

createEvent({
    name: "ready",
    event: "ready",
    async run(client) {
        const status: ActivitiesOptions[] = [
            { name: "Powered by discloud", type:ActivityType.Custom },
            { name: "Join in: .gg/DBNATxA6Jx", type:ActivityType.Custom },
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
            15_000
        );        
    },
});