"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.decomp = void 0;
function decomp(n) {
    const isPrime = (num) => {
        for (var i = 2; i < num; i++) {
            if (num % i === 0)
                return false;
        }
        if (num == 1)
            return false;
        return true;
    };
    const count = {};
    const findPrimeNums = (num) => [...Array(num + 1).keys()].filter((each) => isPrime(each));
    const primeNums = findPrimeNums(n);
    primeNums.forEach((e) => (count[e] = 0));
    const findFactorial = (num) => {
        let factor = 1;
        for (let i = 1; i <= num; i++) {
            factor = factor * i;
        }
        return factor;
    };
    findFactorial(1);
    const Solve = (factor) => {
        let factors = [];
        let arr = [...Array(factor + 1).keys()];
        arr.splice(0, 2);
        console.log("Line 34: ", arr);
        for (const num of arr) {
            if (isPrime(num)) {
                console.log(num);
                count[num] = count[num] + 1;
                factors.push(num);
                return;
            }
            else {
                Solve(num);
            }
            console.log("Count:", count, "\nNum: ", num);
        }
        return;
    };
    let x = Solve(n);
    console.log(x);
    return JSON.stringify(count);
}
exports.decomp = decomp;
console.log("\n05: ", decomp(5) == "2^3 * 3 * 5", "------------", "\n\t\t----Finished----");
//# sourceMappingURL=method2.js.map