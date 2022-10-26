/* Binary Search: https://en.wikipedia.org/wiki/Binary_search_algorithm
 *
 * Search a sorted array by repeatedly dividing the search interval
 * in half. Begin with an interval covering the whole array. If the value of the
 * search key is less than the item in the middle of the interval, narrow the interval
 * to the lower half. Otherwise narrow it to the upper half. Repeatedly check until the
 * value is found or the interval is empty.
 */

type Index = number;
export const binarySearch = <T extends string | number>(arr: Array<T>, x: T): Index | Error => {

  if (arr.length <= 0) {
    return RangeError("Array is empty");
  }

  const mid = Math.floor(arr.length / 2); 

  if (arr[mid] == x) return mid;
  const left = arr.slice(0, mid);
  const right = arr.slice(mid, arr.length - 1);

  switch (typeof arr[mid]) {
    case 'number': 
      if (arr[mid] < x) return binarySearch(left, x);
      if (arr[mid] > x) return binarySearch(right, x);
      break;
    case 'string':
      if (arr[mid] < x) return binarySearch(left, x);
      if (arr[mid] > x) return binarySearch(right, x);
      break;
    default:
      return TypeError("Array must contain type number or string");
  }
      return -1;
}

