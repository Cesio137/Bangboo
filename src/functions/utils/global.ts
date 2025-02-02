import { Canvas, loadImage } from "@napi-rs/canvas";
import {
  AttachmentBuilder,
  ChannelType,
  Events,
  GuildMember,
  PartialGuildMember,
} from "discord.js";
import { join } from "path";

export async function globalMessage(
  event: Events,
  member: GuildMember | PartialGuildMember,
) {
  const canvas = new Canvas(1024, 260);
  const context = canvas.getContext("2d");

  const background = await loadImage(
    join(
      __rootname,
      `assets/images/${event === Events.GuildMemberAdd ? "join" : "leave"}.png`,
    ),
  );
  context.drawImage(background, 0, 0, canvas.width, canvas.height);
  context.save();

  const avatar = await loadImage(member.displayAvatarURL({ size: 256 }));
  context.beginPath();
  context.arc(90 + 68, 90 + 41, 90, 0, Math.PI * 2);
  context.clip();
  context.drawImage(avatar, 68, 41, 180, 180);
  context.restore();

  const actionIconPath = join(
    __rootname,
    `assets/icons/static/${event === Events.GuildMemberAdd ? "add" : "minus"}.svg`,
  );
  const actionIcon = await loadImage(actionIconPath);
  context.drawImage(actionIcon, 205, 179);

  const { displayName, user, joinedTimestamp } = member;
  const { username } = user;

  if (event === Events.GuildMemberAdd) {
    const accountAge = Date.now() - user.createdTimestamp;
    const joinAge = Date.now() - (joinedTimestamp || 0);
    const timeLimit = 60 * 1000;
    const welcomeText =
      joinAge < timeLimit && accountAge > timeLimit
        ? "FIRST TIME"
        : "WELCOME BACK";
    context.fillStyle = "#FFFFFF";
    context.font = "semibold 16px Poppins";
    context.textBaseline = "middle";
    const xPos = welcomeText === "FIRST TIME" ? 556 : 533;
    context.fillText(welcomeText, xPos, 66 + 8);
  }

  let usernameFontSize = 60;
  context.fillStyle = "#FFFFFF";
  do {
    context.font = `bold ${--usernameFontSize}px Rubik`;
  } while (context.measureText(username).width > canvas.width - 400);
  context.textBaseline = "middle";
  context.fillText(username, 300, 110 + usernameFontSize / 2);

  let displayNameFontSize = 32;
  context.fillStyle = "#FFFFFF";
  do {
    context.font = `regular ${--displayNameFontSize}px Lato`;
  } while (context.measureText(`@${displayName}`).width > canvas.width - 400);
  context.textBaseline = "middle";
  context.fillText(`@${displayName}`, 300, 170 + displayNameFontSize / 2);

  const buffer = await canvas.encode("png");
  const attachment = new AttachmentBuilder(buffer, { name: "card.png" });

  const { guild } = member;
  const channel = guild.channels.cache.find((ch) => ch.name === "😏┊welcome");
  if (channel?.type !== ChannelType.GuildText) {
    console.log("Channel not found");
    return;
  }
  channel.send({ files: [attachment] });
}