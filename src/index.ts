import { bootstrap } from "#base";
import { GlobalFonts } from "@napi-rs/canvas";
import { join } from "node:path";

GlobalFonts.loadFontsFromDir(join(__dirname, "assets/fonts"));
await bootstrap({ meta: import.meta });