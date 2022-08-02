"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.decomp = void 0;
function decomp(n) {
    const isPrime = (num) => {
        for (var i = 2; i < num; i++)
            if (num % i === 0)
                return false;
        return num > 1;
    };
    const count = {};
    const primeNums = [...Array(n + 1).keys()].filter((n) => isPrime(n));
    primeNums.forEach((e) => (count[e] = 0));
    const findFactorial = (num) => {
        let factor = 1;
        for (let i = 1; i <= num; i++) {
            factor = factor * i;
        }
        return factor;
    };
    const Solve = (factorial, count) => {
        Object.keys(count).forEach((num) => {
            let each = Number(num);
            while (factorial % each == 0) {
                console.log(`div: ${each} | fact: ${factorial} | rem: ${factorial % each}`);
                count[each] = count[each] + 1;
                factorial = factorial / each;
            }
        });
    };
    Solve(findFactorial(n), count);
    console.log("Count: ", count);
    return Object.keys(count)
        .map((idx) => {
        if (count[Number(idx)] != 0 && count[Number(idx)] != 1) {
            return `${idx}^${count[Number(idx)]}`;
        }
        else {
            return `${idx}`;
        }
    })
        .join(" * ");
}
exports.decomp = decomp;
console.log("\n05: ", decomp(5) == "2^3 * 3 * 5", "------------", "\n25: ", decomp(25) == "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23", "\n\t\t----Finished----");
//# sourceMappingURL=method1.js.map