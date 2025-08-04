import { env } from "#settings";
import { discloud } from "discloud.app";

export const appID = "1754339178532";
export const user = await discloud.login(env.DISCLOUD_TOKEN);