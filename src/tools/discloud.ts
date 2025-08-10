import { env } from "#env";
import { discloud } from "discloud.app";

export const appID = "1754869372058";
export const user = await discloud.login(env.DISCLOUD_TOKEN);