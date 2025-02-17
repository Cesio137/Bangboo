import { DefaultExtractors } from "@discord-player/extractor";
import { Player } from "discord-player";
import { YoutubeiExtractor } from "discord-player-youtubei";
import { Client } from "discord.js";


export function baseLoadDiscordPlayerExtractors(client: Client<boolean>) {
    client.player = new Player(client as never);
    client.player.extractors.loadMulti(DefaultExtractors);
    client.player.extractors.register(YoutubeiExtractor, {});
}