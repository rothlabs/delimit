import init, * as app from 'init';
async function run() {
    console.log("here");
    await init({});
    console.log("worked");
    // now call wasm methods on * as app as desired
}
setTimeout(() => {
    run();
}, 3000);
  