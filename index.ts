const [DARWIN, GNU, WIN] = ['darwin', 'linux', 'win32'];
const osdata = {
  [DARWIN]: 'darwin',
  [GNU]: 'gnu',
  [WIN]: 'win'
};
import os from 'os';
const osmain = os.platform();
const fileRequire = './index.' + osdata[osmain] + '.node';
const rsBind: Record<string, Function> = require(fileRequire);

type LuaLib = "base"|"bit32"|"coroutine"|"debug"|"io"|"math"|"os"|"package"|"string"|"table"|"utf8";

class LuaState
{
    private state: any;

    constructor() { this.state = rsBind.luaNew(); }

    loadLibs(libs?: LuaLib[])
    {
        if (libs) rsBind.luaOpenLibs.call(this.state, libs);
        else rsBind.luaOpenLibs.call(this.state);
    }
    loadLib(lib: LuaLib) { rsBind.luaOpenLib.call(this.state, lib); }

    getTop(): number { return rsBind.luaGetTop.call(this.state); }
    //getGlobal(name: string): any { return rsBind.luaGetGlobal.call(this.state, name); }

    pushNumber(num: number) { rsBind.luaPushNumber.call(this.state, num); }
    setGlobal(name: string) { rsBind.luaSetGlobal.call(this.state, name); }
    createGlobal(name: string, value: string|number|boolean) { rsBind.luaCreateGlobal.call(this.state, name, value); }

    doString(code: string) { rsBind.luaDoString.call(this.state, code); }
    doFile(file: string) { rsBind.luaDoFile.call(this.state, file); }
}

const luaRun = rsBind.luaRun;
const luaSafe = [
    "base",
    "bit32",
    "coroutine",
    "debug",
    "math",
    "string",
    "table",
    "utf-8"
];

let testState = new LuaState();
testState.loadLibs();
testState.createGlobal("test", 600);
testState.doString("print(test)");
//testState.getGlobal("test");

export {
    LuaState,
    luaRun,
    luaSafe,
}