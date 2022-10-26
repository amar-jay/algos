"use strict";
/* Binary Search: https://en.wikipedia.org/wiki/Binary_search_algorithm
 *
 * Search a sorted array by repeatedly dividing the search interval
 * in half. Begin with an interval covering the whole array. If the value of the
 * search key is less than the item in the middle of the interval, narrow the interval
 * to the lower half. Otherwise narrow it to the upper half. Repeatedly check until the
 * value is found or the interval is empty.
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.binarySearch = void 0;
const binarySearch = (arr, x) => {
    if (arr.length <= 0) {
        return RangeError("Array is empty");
    }
    if (arr.length == 1) {
        return arr[0] == x ? 0 : -1;
    }
    const mid = Math.floor(arr.length / 2);
    if (arr[mid] == x)
        return mid;
    const left = arr.slice(0, mid);
    const right = arr.slice(mid, arr.length - 1);
    switch (typeof arr[mid]) {
        case 'number':
            if (arr[mid] < x)
                return (0, exports.binarySearch)(left, x);
            if (arr[mid] > x)
                return (0, exports.binarySearch)(right, x);
            break;
        case 'string':
            if (arr[mid] < x)
                return (0, exports.binarySearch)(left, x);
            if (arr[mid] > x)
                return (0, exports.binarySearch)(right, x);
            break;
        default:
            return TypeError("Array must contain type number or string");
    }
    return -1;
};
exports.binarySearch = binarySearch;
