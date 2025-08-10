import { validateEnv } from "#base";
import { z } from "zod";

export const env = validateEnv(z.object({
    BOT_TOKEN: z.string("Discord Bot Token is required").min(1),
    WEBHOOK_LOGS_URL: z.url().optional(),
    DISCLOUD_TOKEN: z.string("Discloud token is required").min(1),
    GEMINI_API_KEY: z.string("Gemini API key is required").min(1),
}));