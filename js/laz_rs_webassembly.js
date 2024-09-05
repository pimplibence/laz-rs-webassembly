export default class LAZRSWebAssemblyMain {
    libPath = '/pkg/laz_rs_webassembly-lib.js';
    workerPath = '/pkg/laz_rs_webassembly-worker.js';
    wasmPath = '/pkg/laz_rs_webassembly-lib_bg.wasm';
    worker;

    constructor(options) {
        if (options?.libPath) {
            this.libPath = options.libPath;
        }

        if (options?.workerPath) {
            this.workerPath = options.workerPath;
        }

        if (options?.wasmPath) {
            this.wasmPath = options.wasmPath;
        }
    }

    read(dataUint8Array) {
        const worker = new Worker(this.workerPath);

        return new Promise((resolve, reject) => {
            worker.onmessage = (data) => {
                resolve(data.data);
                worker.terminate();
            }

            worker.onerror = (err) => {
                reject(err);
                worker.terminate();
            }

            worker.onmessageerror = (err) => {
                reject(err);
                worker.terminate();
            }

            worker.postMessage({
                workerPath: this.workerPath,
                wasmPath: this.wasmPath,
                libPath: this.libPath,
                data: dataUint8Array
            });
        });
    }
}
