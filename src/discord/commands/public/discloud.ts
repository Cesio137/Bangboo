import { createCommand } from "#base";
import { icon, res } from "#functions";
import { user } from "#tools";
import { ApplicationCommandOptionType, ApplicationCommandType, ChatInputCommandInteraction } from "discord.js";
import { createEmbed } from "@magicyan/discord";

createCommand({
    name: "discloud",
    description: "app command",
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

    await interaction.deferReply();

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
    const embed = createEmbed({
        author: {
            name: app.name,
            iconURL: app.avatarURL
        },
        color: "Green",
        thumbnail: app.avatarURL,
        description: `${infos.join("\n")}`
    })

    interaction.editReply({ embeds: [embed] });
}

async function logs(interaction: ChatInputCommandInteraction<"cached">) {
    const id = user.appIDs.values().next().value;
    if (!id) {
        interaction.reply(res.warning("Failed to fetch app status."));
        return;
    }

    await interaction.deferReply();
    
    const app = await user.apps.fetch(id);
    const appLogs = await user.apps.terminal(id);
    
    const embed = createEmbed({
        author: {
            name: app.name,
            iconURL: app.avatarURL
        },
        color: "Green",
        description: `\`\`\`bash\n${appLogs.small}\n\`\`\``
    })

    interaction.editReply({ embeds: [embed] });
}