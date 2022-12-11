class Algos {
  static int binarySearch(List<int> arg, int val) {
    int first = 0;
    int last = arg.length;

    while (first <= last) {
      int mid = ((first + last ) /2).round();

      if (arg[mid] == val) {
	  return mid;
      }

      if (arg[mid] > val) {
      last = mid + 1; 
      continue;
    } 
      first = mid - 1;
    }
    return 0;
  }

   static List<int> mergeSort(List<int> arr) {
    int start = 0;
    int end = arr.length - 1;
    int mid = (arr.length /2).floor();

    if (start > end ) {
      return arr;
    }
    List<int> left = mergeSort(arr.sublist(0, mid + 1));
    List<int> right = mergeSort(arr.sublist(mid,));

    arr = [...left, ...right];
    return arr;
  }
}
