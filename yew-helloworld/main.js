import init, { run_app } from './pkg/yew_helloworld.js';
async function main() {
   await init('/pkg/yew_helloworld_bg.wasm');
   run_app();
}
main();
