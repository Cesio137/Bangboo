import settings from "../../settings.json" with { type: "json" };
import emojis from "../../data/emojis.json" with { type: "json" };
import guild from "../../data/guild.json" with { type: "json" };
import fab from "../../data/fab.json" with { type: "json" };
import { envSchema } from "./env.schema.js";

import "./global.js";
import { logger } from "./logger.js";
import { validateEnv } from "./env.validate.js";
export * from "./error.js";

const env = validateEnv(envSchema);

export { settings, emojis, guild, fab, logger, env };