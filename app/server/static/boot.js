import init, * as app from 'init';
init('client_bg.wasm').then(() => {
    app.initialize();
});