import tailwindcss from "@tailwindcss/vite";
import adapter from "@sveltejs/adapter-static";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
    plugins: [
        tailwindcss(),
        sveltekit({
            files: {
                src: "src-svelte",
            },

            compilerOptions: {
                runes: ({ filename }) =>
                    filename.split(/[/\\]/).includes("node_modules") ? undefined : true,
            },

            adapter: adapter({
                pages: "dashboard",
                assets: "dashboard",
                fallback: "index.html",
            }),
        }),
    ],
});
