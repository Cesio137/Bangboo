import { Command } from "#base";
import { createQueueMetadata, icon, res } from "#functions";
import { settings } from "#settings";
import { brBuilder, createEmbed } from "@magicyan/discord";
import { multimenu } from "@magicyan/discord-ui";
import { QueryType, SearchQueryType } from "discord-player";
import { ApplicationCommandOptionType, ApplicationCommandType } from "discord.js";

new Command({
    name: "player",
    description: "Commands to control your podcasts and OST queue!",
    type: ApplicationCommandType.ChatInput,
    options: [
        {
            name: "add",
            description: "Add your favorite podcast or OST to queue!",
            type: ApplicationCommandOptionType.Subcommand,
            options: [
                {
                    name: "search",
                    description: "Enter a name or url.",
                    type: ApplicationCommandOptionType.String,
                    required: true,
                },
                {
                    name: "engine",
                    description: "Search engine(optional).",
                    type: ApplicationCommandOptionType.String,
                    choices: Object.values(QueryType).map((type) => ({ name: type, value: type })),
                },
            ],
        },
        {
            name: "pause",
            description: "Pause the current track!",
            type: ApplicationCommandOptionType.Subcommand,
        },
        {
            name: "resume",
            description: "Resume the current track!",
            type: ApplicationCommandOptionType.Subcommand,
        },
        {
            name: "skip",
            description: "Skip a certain numbers of track!",
            type: ApplicationCommandOptionType.Subcommand,
            options: [
                {
                    name: "amount",
                    description: "Amount of tracks to skip.",
                    type: ApplicationCommandOptionType.Integer,
                },
            ],
        },
        {
            name: "stop",
            description: "Stop and clean the queue!",
            type: ApplicationCommandOptionType.Subcommand,
        },
        {
            name: "queue",
            description: "Show the tracks on the queue.",
            type: ApplicationCommandOptionType.Subcommand,
        },
    ],
    async run(interaction) {
        const { options, member, guild, channel, client } = interaction;
        if (!member.voice.channel)
            return interaction.reply(
                res.danger(`${icon.close} You are not connected to a voice channel!`)
            );
        if (!channel)
            return interaction.reply(
                res.danger(`${icon.close} It is not possible to use this command on this channel.`)
            );

        await interaction.deferReply();
        const queue = client.player.queues.cache.get(guild.id);
        if (options.getSubcommand(true) !== "add" && !queue)
            return interaction.editReply(
                res.danger(`${icon.close} There is no track on the queue!`)
            );

        const voiceChannel = member.voice.channel;
        const queueMetadata = createQueueMetadata({ channel, client, guild, voiceChannel });

        switch (options.getSubcommand(true)) {
            case "add":
                try {
                    const query = options.getString("search", true);
                    const searchEngine = options.getString("engine") ?? QueryType.YOUTUBE;

                    const { track, searchResult } = await client.player.play(
                        voiceChannel as never,
                        query,
                        {
                            searchEngine: searchEngine as SearchQueryType,
                            nodeOptions: { metadata: queueMetadata },
                        }
                    );

                    const display: string[] = [];

                    if (searchResult.playlist) {
                        const { tracks, title, url } = searchResult.playlist;
                        display.push(
                            `Added ${tracks.length} tracks from playlist [${title}](${url}).`,
                            ...tracks.map((track) => `${track.title}`).slice(0, 8),
                            "..."
                        );
                    } else {
                        display.push(
                            `${queue?.size ? "Added to queue. " : "Playing now! "} ${track.title}`
                        );
                    }
                    return interaction.editReply(
                        res.success(`${icon.check} ${brBuilder(display)}`)
                    );
                } catch (e) {
                    return interaction.editReply(
                        res.danger(`${icon.close} Error when trying to play the track.\n${e}`)
                    );
                }
                return;
            case "pause":
                if (queue?.node.isPaused())
                    return interaction.editReply(
                        res.danger(`${icon.close} The current track is already paused!`)
                    );
                queue?.node.pause();
                return interaction.editReply(
                    res.success(`${icon.check} Current track has been paused!`)
                );
            case "resume":
                if (!queue?.node.isPaused())
                    return interaction.editReply(
                        res.danger(`${icon.close} The current track is not paused!`)
                    );
                queue.node.resume();
                return interaction.editReply(
                    res.success(`${icon.check} Current track has been resumed!`)
                );
            case "stop":
                queue?.node.stop();
                return interaction.editReply(
                    res.success(
                        `${icon.check} Current track has been stopped and track list has been cleaned!`
                    )
                );
            case "skip":
                const amount = options.getInteger("amount") ?? 1;
                const skipAmount = Math.min(queue!.size, amount);
                for (let i = 0; i < skipAmount; i++) {
                    queue?.node.skip();
                }
                return interaction.editReply(
                    res.success(
                        `${icon.check} ${skipAmount} ${
                            skipAmount > 1 ? "tracks have been skipped!" : "track has been skipped!"
                        } `
                    )
                );
            case "queue":
                multimenu({
                    embed: createEmbed({
                        color: settings.colors.fuchsia,
                        description: brBuilder(
                            "# Current queue",
                            `Amount: ${queue!.tracks.size}`,
                            "",
                            `Current track: ${queue!.currentTrack?.title ?? "Nothing"}`
                        ),
                    }),
                    items: queue!.tracks.map((track) => ({
                        color: settings.colors.green,
                        description: brBuilder(
                            `**Title**: [${track.title}](${track.url})`,
                            `**Autor**: ${track.author}`,
                            `**Duration**: ${track.duration}`
                        ),
                        thumbnail: track.thumbnail,
                    })),
                    render: (embeds, components) => interaction.editReply({ embeds, components }),
                });
                return;
        }
        return;
    },
});
