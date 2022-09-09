"use strict";
exports.__esModule = true;
exports.sumOfIntervals = void 0;
function sumOfIntervals(intervals) {
    var ans = new Set();
    intervals.map(function (range) {
        var arr = Array.from({ length: range[1] - range[0] }, function (_, k) { return k + range[0]; });
        arr.map(function (x) { return ans.add(x); });
    });
    return ans.size;
}
exports.sumOfIntervals = sumOfIntervals;
function maintests(func) {
    console.log("\t\t\t----TESTS 🧪----", "\n✔️ 1st: ", func([[1, 5]]) === 4, "\n✔️ 2nd: ", func([
        [1, 5],
        [6, 10],
    ]) === 8, "\n✔️ 3rd: ", func([
        [1, 5],
        [1, 5],
    ]) === 4, "\n✔️ 4th: ", func([
        [1, 4],
        [7, 10],
        [3, 5],
    ]) === 7, "\n\t\t----Finished----");
}
maintests(sumOfIntervals);
