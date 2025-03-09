# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


# Get start

Install Rust dependencies
```
cargo install create-tauri-app --locked
cargo install tauri-cli --locked
```


Install npm dependencies

```
npm install
```

Update tailwind css
```
npx tailwindcss -i ./src/input.css -o ./src/output.css --watch
```

# Windows dev

Follow: 

https://v2.tauri.app/start/prerequisites/#windows



## Install dependencies on windows

choco
```
choco install llvm opencv
```

camera related:
https://github.com/cameleon-rs/cameleon#roadmap


after install those lib, config PATH env and restart software

https://github.com/twistedfall/opencv-rust/blob/master/TROUBLESHOOTING.md


