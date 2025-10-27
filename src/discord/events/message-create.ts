import { createEvent } from "#base";
import { filterAttachment } from "#tools";

createEvent({
    name: "Message Create",
    event: "messageCreate",
    async run(message) {
        if (message.author.bot || !message.guild) return;
        filterAttachment(message);
    },
});
