import { settings } from "#settings";

type ChannelList = typeof settings.channels;

const channels: ChannelList = Object.create({});

for (const [name, id] of Object.entries(settings.channels)) {
    Object.assign(channels, { [name]: id });
}

type RoleList = typeof settings.roles;

const roles: RoleList = Object.create({});

for (const [name, id] of Object.entries(settings.roles)) {
    Object.assign(roles, { [name]: id });
}

export { channels, roles }