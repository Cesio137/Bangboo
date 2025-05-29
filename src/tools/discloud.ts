import { env } from "#settings";
import { discloud } from "discloud.app";

export const user = await discloud.login(env.DISCLOUD_TOKEN);