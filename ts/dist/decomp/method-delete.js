"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const path_1 = __importDefault(require("path"));
const filePath = (filename) => {
    return path_1.default.join(__dirname, filename);
};
let x = filePath('../hello.txt');
console.log(x);
