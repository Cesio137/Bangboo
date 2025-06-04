import { guild } from "#settings";

type GuildList = typeof guild.channels;

const channels: GuildList = Object.create({});

for (const [name, id] of Object.entries(guild.channels)) {
    Object.assign(channels, { [name]: id });
}

type RoleList = typeof guild.roles;

const roles: RoleList = Object.create({});

for (const [name, id] of Object.entries(guild.roles)) {
    Object.assign(roles, { [name]: id });
}

export { channels, roles }