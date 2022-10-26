"use strict";
/*
 *Solve as long division
 * NOT WORKING
 * */
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
    //   return JSON.stringify(count)
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
console.log(
//"17: ", decomp(17) == "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17",
"\n05: ", decomp(5) == "2^3 * 3 * 5", "------------", 
//"\n22: ", decomp(22) == "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19",
//"\n14: ", decomp(14) == "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13",
"\n25: ", decomp(25) == "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23", "\n\t\t----Finished----");
