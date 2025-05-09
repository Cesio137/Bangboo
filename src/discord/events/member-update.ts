import { createEvent } from "#base";
import { globalBoost, roles } from "#functions"

createEvent({
    name: "Member Update",
    event: "guildMemberUpdate",
    async run(oldMember, newMember) {
        if (!oldMember.premiumSince && newMember.premiumSince) {
            newMember.roles.add(roles.boosters);
            globalBoost(newMember);
        } else {
            newMember.roles.remove(roles.boosters);
        }
    }
});