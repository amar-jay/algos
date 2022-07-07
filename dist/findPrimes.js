function Zeros(n) {
    let count = 0;
    while (n > 0) {
        n = n / 5;
        count = count + Math.floor(n);
    }
    return count;
}
console.log("17:", Zeros(17), findFactorial(17));
console.log("18:", Zeros(18), "Expected: 2", findFactorial(18));
console.log("9:", Zeros(9), findFactorial(9));
console.log("10:", Zeros(10), findFactorial(10));
function findFactorial(n) {
    let factor = 1;
    for (let i = 1; i <= n; i++) {
        factor = factor * i;
    }
    return factor;
}
//# sourceMappingURL=findPrimes.js.map