import { bootstrapApp } from "#base";
import { GlobalFonts } from "@napi-rs/canvas";
import { join } from "node:path";

GlobalFonts.loadFontsFromDir(join(__rootname, "assets/fonts"));
await bootstrapApp({ workdir: import.meta.dirname });
