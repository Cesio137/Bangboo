import { createCommand } from "#base";
import { icon, res } from "#functions";
import { user } from "#tools";
import { ApplicationCommandOptionType, ApplicationCommandType, ChatInputCommandInteraction } from "discord.js";
import { logsComponent, statusComponent } from "#menus";

createCommand({
    name: "discloud",
    description: "Show the bot status on discloud host",
    type: ApplicationCommandType.ChatInput,
    options: [
        {
            name: "fetch",
            description: "command option",
            required: true,
            type: ApplicationCommandOptionType.String,
            choices: [
                { name: "status", value: "status" },
                { name: "logs", value: "logs" },
            ]
        }
    ],
    async run(interaction) {
        const { options } = interaction;

        switch (options.getString("fetch")) {
            case "status":
                status(interaction);
                break;
            case "logs":
                logs(interaction);
                break;
        }
    }
});

async function status(interaction: ChatInputCommandInteraction<"cached">) {
    const id = user.appIDs.values().next().value;
    if (!id) {
        interaction.reply(res.warning("Failed to fetch app status."));
        return;
    }
    
    await interaction.deferReply({ flags: ["Ephemeral"] });

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

    interaction.editReply({ flags: ["IsComponentsV2"], components: [component] });
}

async function logs(interaction: ChatInputCommandInteraction<"cached">) {
    const id = user.appIDs.values().next().value;
    if (!id) {
        interaction.reply(res.warning("Failed to fetch app status."));
        return;
    }

    await interaction.deferReply({ flags: ["Ephemeral"] });

    const appLogs = await user.apps.terminal(id);
    let logs = appLogs.small.length > 3000 ? appLogs.small.slice(0, 3000) : appLogs.small; 
    logs = logs.replace(/\[[0-9;]+m/g, '');

    const component = logsComponent(logs);

    interaction.editReply({ flags: ["IsComponentsV2"], components: [component] });
}