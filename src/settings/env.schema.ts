import { z } from "zod";

export const envSchema = z.object({
    BOT_TOKEN: z.string({ description: "Discord Bot Token is required" }).min(1),
    WEBHOOK_LOGS_URL: z.string().url().optional(),
    // Env vars...
    DISCLOUD_TOKEN: z.string({ description: "Discloud token is required" }).min(1),
    GEMINI_API_KEY: z.string({ description: "Gemini API key is required" }).min(1),
});