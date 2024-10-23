import { Command } from "#base";
import { icon, res } from "#functions";
import { ApplicationCommandOptionType, ApplicationCommandType } from "discord.js";

new Command({
	name: "age",
	description: "Displays your or another user's account creation date.",
	type: ApplicationCommandType.ChatInput,
    options: [
        {
            name: "user",
            type: ApplicationCommandOptionType.User,
            description: "Selected user."
        }
    ],
	run(interaction){
        const { options } = interaction;
        const user = options.getUser("user") || interaction.user;
        const { username, createdAt } = user;
        const date = createdAt.toUTCString().split(' ');
        const age = `${username}'s account was created at ${date[1]}/${date[2]}/${date[3]} ${date[4]}`;
		interaction.reply(res.green(age));
	}
});