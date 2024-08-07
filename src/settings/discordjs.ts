import { Player } from "discord-player";

declare module "discord.js" {
	interface Client {
		// Add your properties
		player: Player;
	}
}