/*
import { properties } from "#settings";
import { Player } from "discord-player";
import { Client } from "discord.js";


export function baseLoadDiscordPlayerExtractors(client: Client<boolean>) {
    client.player = new Player(
        client as never, 
        { skipFFmpeg: true, lagMonitor: 0 }
    );
    client.player.extractors.loadMulti(properties.extractors);
}
*/