import { bootstrap } from "#base";
import { GlobalFonts } from "@napi-rs/canvas";

GlobalFonts.loadFontsFromDir("./assets/fonts");
await bootstrap({ meta: import.meta });