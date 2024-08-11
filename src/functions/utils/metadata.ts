import { GuildQueue } from "discord-player";
import { Client, Guild, GuildTextBasedChannel, VoiceBasedChannel } from "discord.js";

interface QueueMetadata {
  client: Client<true>;
  guild: Guild;
  voiceChannel: VoiceBasedChannel;
  channel: GuildTextBasedChannel;
}

export function createQueueMetadata(metadata: QueueMetadata) {
  return metadata;
}

export function getQueueMetadata(queue: GuildQueue): QueueMetadata {
  return queue.metadata as QueueMetadata;
}
