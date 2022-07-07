"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.decomp = void 0;
function decomp(number) {
    const subdecomp = (number, subdividers) => {
        let remainder = number;
        for (let x = 2; x <= Math.sqrt(number); x++) {
            if (remainder % x === 0) {
                if (!subdividers[x])
                    subdividers[x] = 0;
                while (remainder % x === 0) {
                    subdividers[x]++;
                    remainder = remainder / x;
                }
            }
        }
        if (remainder > 1) {
            if (!subdividers[remainder])
                subdividers[remainder] = 1;
            else
                subdividers[remainder] += 1;
        }
        return subdividers;
    };
    let dividers = {};
    for (let x = 2; x <= number; x++)
        dividers = subdecomp(x, dividers);
    return Object.keys(dividers)
        .map((piece) => dividers[Number(piece)] !== 1
        ? piece + "^" + dividers[Number(piece)]
        : piece, "SOLUTION: ")
        .join(" * ");
}
exports.decomp = decomp;
console.clear();
console.log("\t\t\t----TESTS 🧪----", "\n17: ", decomp(17) == "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17", "\n05: ", decomp(5) === "2^3 * 3 * 5", "\n22: ", decomp(22) == "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19", "\n14: ", decomp(14) == "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13", "\n25: ", decomp(25) == "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23", "\n\t\t----Completed Successfully 🍾----");
//# sourceMappingURL=method3.js.map