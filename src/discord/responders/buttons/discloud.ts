import { createResponder, ResponderType } from "#base";
import { res, icon } from "#functions";
import { logsComponent, statusComponent } from "#menus";
import { user } from "#tools";

createResponder({
    customId: "discloud/status/refresh",
    types: [ResponderType.Button],
    async run(interaction) {
        const id = user.appIDs.values().next().value;
        if (!id) {
            interaction.reply(res.warning("Failed to fetch app status."));
            return;
        }

        await interaction.deferUpdate();

        const app = await user.apps.fetch(id);
        const appStatus = await user.apps.status(id);
        const infos = [
            `${icon.id}\`Nome(ID):\` **${app.name}(${app.id})**`,
            `${icon.cpu}\`CPU:\` **${appStatus.cpu}**`,
            `${icon.ram}\`RAM:\` **${appStatus.memory}**`,
            `${icon.ssd}\`SSD:\` **${appStatus.ssd}**`,
            `${icon.wifi}\`Network:\` \`⬆\`**${appStatus.netIO.up} \`⬇\`${appStatus.netIO.down}**`,
            `${icon.refresh}\`Latest restart:\` **<t:${Math.floor(appStatus.startedAtTimestamp / 1000)}:R>**`,
        ];
        const component = statusComponent(infos);
        interaction.editReply({ flags: ["IsComponentsV2"], components: [component] })
    },
});

createResponder({
    customId: "discloud/logs/refresh",
    types: [ResponderType.Button],
    async run(interaction) {
        const id = user.appIDs.values().next().value;
        if (!id) {
            interaction.reply(res.warning("Failed to fetch app status."));
            return;
        }

        await interaction.deferUpdate();

        const appLogs = await user.apps.terminal(id);
        let logs = appLogs.small.length > 3000 ? appLogs.small.slice(0, 3000) : appLogs.small; 
        logs = logs.replace(/\[[0-9;]+m/g, '');
        
        const component = logsComponent(appLogs.small);
        
        interaction.editReply({ flags: ["IsComponentsV2"], components: [component] });
    },
});