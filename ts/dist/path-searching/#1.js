"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.pathFinder = void 0;
function pathFinder(maze) {
    const WALL = "W";
    const PATH = ".";
    let table = maze.split("\n").map((row) => row.split(""));
    let max_len = table.length - 1;
    let stack = [];
    stack.push([0, 0]);
    while (stack.length != 0 || stack != null) {
        let elem = stack.pop();
        if (elem == null) {
            return false;
        }
        let x = elem[0];
        let y = elem[1];
        /*
         * console.log(
              stack,
              x,
              y,
              max_len,
              "right:",
              table[x][y + 1],
              "curr:",
              table[x][y],
              "left:",
              table[x][y - 1]
            );
            */
        if (x === max_len && y === max_len) {
            return true;
        }
        if (table[x][y] === WALL) {
            continue;
        }
        //check right
        if (max_len >= y + 1 && max_len >= x && table[x][y + 1] === PATH) {
            stack.unshift([x, y + 1]);
        }
        // check down
        if (max_len >= y && max_len >= x + 1 && table[x + 1][y] === PATH) {
            stack.unshift([x + 1, y]);
        }
        // check left
        // if (!table[x][y - 1]) {
        //   continue;
        // }
        // if (
        //   max_len >= y - 1 &&
        //   max_len >= x &&
        //   table[x][y - 1] !== undefined &&
        //   table[x][y - 1] === PATH &&
        //   table[x][y - 1] !== WALL
        // ) {
        //   stack.unshift([x, y - 1]);
        // }
        // // check up
        // if (!table[x - 1][y]) {
        //   continue;
        // }
        // if (
        //   max_len >= y &&
        //   max_len >= x - 1 &&
        //   table[x - 1][y] !== undefined &&
        //   table[x - 1][y] === PATH &&
        //   table[x - 1][y] !== WALL
        // ) {
        //   stack.unshift([x - 1, y]);
        // }
    }
    return false;
}
exports.pathFinder = pathFinder;
