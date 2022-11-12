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

    openLibs(libs?: LuaLib[])
    {
        if (libs) rsBind.luaOpenLibs.call(this.state, libs);
        else rsBind.luaOpenLibs.call(this.state);
    }
    openLib(lib: LuaLib) { rsBind.luaOpenLib.call(this.state, lib); }

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

export {
    LuaState,
    luaRun,
    luaSafe,
}