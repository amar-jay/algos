"use strict";
/*
 *Solve by factor tree
 * NOT WORKING
 * */
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
    /*  return Object.keys(count).map((idx) => {
      if (count[Number(idx)]!=0 && count[Number(idx)]!=1 ){
      return `${idx}^${count[Number(idx)]}`
      }else{
        return `${idx}`
      }
      }).join(' * ')
     */
}
exports.decomp = decomp;
console.log(
//"17: ", decomp(17) == "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17",
"\n05: ", decomp(5) == "2^3 * 3 * 5", "------------", 
//"\n22: ", decomp(22) == "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19",
//"\n14: ", decomp(14) == "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13",
//"\n25: ", decomp(25) == "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23"
"\n\t\t----Finished----");
