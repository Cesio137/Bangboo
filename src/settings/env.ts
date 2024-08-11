import { z } from "zod";

const envSchema = z.object({
    BOT_TOKEN: z.string({ description: "Discord Bot Token is required" }).min(1),
    WEBHOOK_LOGS_URL: z.string().url().optional(),
    //MONGO_URI: z.string({ description: "MongoDb URI is required" }).min(1),
    // Env vars...
    //CLIENT_ID: z.string(),
    //GUILD_ID: z.string(),
    GEMINI_API_KEY: z.string(),
});

type EnvSchema = z.infer<typeof envSchema>;

export { envSchema, type EnvSchema };