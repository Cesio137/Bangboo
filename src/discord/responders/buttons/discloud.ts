import { createResponder, ResponderType } from "#base";
import { logsComponent, statusComponent } from "#menus";
import { appID, user } from "#tools";

createResponder({
    customId: "discloud/status/refresh",
    types: [ResponderType.Button],
    async run(interaction) {
        await interaction.deferUpdate();

        const app = await user.apps.fetch(appID);
        const appStatus = await user.apps.status(appID);
        const infos = [
            `<:id:${emojis.static.id}>\`Nome(ID):\` **${app.name}(${app.id})**`,
            `<:cpu:${emojis.static.cpu}>\`CPU:\` **${appStatus.cpu}**`,
            `<:ram:${emojis.static.ram}>\`RAM:\` **${appStatus.memory}**`,
            `<:ssd:${emojis.static.ssd}>\`SSD:\` **${appStatus.ssd}**`,
            `<:wifi:${emojis.static.wifi}>\`Network:\` \`⬆\`**${appStatus.netIO.up} \`⬇\`${appStatus.netIO.down}**`,
            `<:resfresh:${emojis.static.refresh}>\`Latest restart:\` **<t:${Math.floor(appStatus.startedAtTimestamp / 1000)}:R>**`,
        ];
        const component = statusComponent(infos);
        interaction.editReply({ flags: ["IsComponentsV2"], components: [component] })
    },
});

createResponder({
    customId: "discloud/logs/refresh",
    types: [ResponderType.Button],
    async run(interaction) {
        await interaction.deferUpdate();

        const appLogs = await user.apps.terminal(appID);
        let logs = appLogs.small.length > 3000 ? appLogs.small.slice(0, 3000) : appLogs.small;
        logs = logs.replace(/\x1b\[[0-9;]*m/g, '');

        const component = logsComponent(logs);

        interaction.editReply({ flags: ["IsComponentsV2"], components: [component] });
    },
});