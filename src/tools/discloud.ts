import { env } from "#env";
import { discloud } from "discloud.app";
import fs from "node:fs"
import z from "zod";

function parseConfigFile(content: string): Record<string, string> {
    return Object.fromEntries(
        content
            .split('\n')
            .map(line => line.trim())
            .filter(line => line && !line.startsWith('#'))
            .map(line => {
                const [key, ...vals] = line.split('=');
                return [key, vals.join('=').trim()];
            })
    );
}

function discloudAppID(): string {
    const filePath = "./discloud.config";
    const content = fs.readFileSync(filePath, "utf8");
    const configObj = parseConfigFile(content);

    const configSchema = z.object({
        ID: z.string().optional()
    });

    const result = configSchema.safeParse(configObj);

    console.log(result.data);

    if (!result.success) return "";
    return typeof result.data.ID !== "undefined" ? result.data.ID : "";
}

export const appID = discloudAppID();
export const user = await discloud.login(env.DISCLOUD_TOKEN);