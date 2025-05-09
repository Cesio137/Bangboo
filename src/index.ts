import { bootstrap } from "#base";
import { GlobalFonts } from "@napi-rs/canvas";
import { join } from "node:path";

if (global.gc) {
    setInterval(
        function() {
            if (global.gc) global.gc();
        },
        300000
    )
}
GlobalFonts.loadFontsFromDir(join(__rootname, "assets/fonts"));
await bootstrap({ meta: import.meta });