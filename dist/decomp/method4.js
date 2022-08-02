const findFactorial = (num) => {
    return [...new Array(num + 1).fill(0).keys()].filter((e) => e > 1);
};
const isPrime = (num) => num <= 1
    ? false
    : !Array.from(new Array(num), (_, i) => i + 1)
        .filter((x) => x > 1 && x < num)
        .find((x) => num % x === 0);
const primeNums = (num) => {
    let list = [...new Array(num + 1).fill(0).keys()].filter((num) => isPrime(num));
    let primes = {};
    list.forEach((e) => (primes[e] = 0));
    return primes;
};
function main(num) {
    let factorial = findFactorial(num);
    let primeNumbers = primeNums(num);
    for (let prime of Object.keys(primeNumbers)) {
        for (let i = 0; i < factorial.length; i++) {
            let num = factorial[i];
            while (num % Number(prime) === 0) {
                primeNumbers[Number(prime)] = primeNumbers[Number(prime)] + 1;
                num /= Number(prime);
            }
        }
    }
    let str = Object.keys(primeNumbers)
        .map((a) => a + (primeNumbers[Number(a)] > 1 ? `^${primeNumbers[Number(a)]}` : ""))
        .join(" * ");
    return str;
}
function maintests(func) {
    console.clear();
    console.log("\t\t\t----TESTS 🧪----", "\n✔️ 17: ", func(17) == "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17", "\n✔️ 05: ", func(5) == "2^3 * 3 * 5", "\n✔️ 22: ", func(22) == "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19", "\n✔️ 14: ", func(14) == "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13", "\n✔️ 25: ", func(25) == "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23", "\n\t\t----Finished----");
}
maintests(main);
//# sourceMappingURL=method4.js.map