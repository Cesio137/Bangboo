import { env } from "#env";
import { discloud } from "discloud.app";

export const appID = "1757684892025";
export const user = await discloud.login(env.DISCLOUD_TOKEN);