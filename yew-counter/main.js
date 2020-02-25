import init, { run_app } from './pkg/yew_counter.js';
async function main() {
   await init('/pkg/yew_counter_bg.wasm');
   run_app();
}
main();
