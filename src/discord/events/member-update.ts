import { createEvent } from "#base";
import { globalBoost } from "#functions"

createEvent({
    name: "Member Update",
    event: "guildMemberUpdate",
    async run(oldMember, newMember) {
        if (!oldMember.premiumSince && newMember.premiumSince) {
            newMember.roles.add(guildData.roles.boosters);
            globalBoost(newMember);
        } else {
            newMember.roles.remove(guildData.roles.boosters);
        }
    }
});