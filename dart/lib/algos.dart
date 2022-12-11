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
    int mid = (arr.length/2).floor();

    if ( arr.length == 2) {
      if (arr[0] > arr[1]) {
	return arr;
      }
      return [arr[1], arr[0]];
    }

    if ( arr.length < 2) {
      return arr;
    }
    List<int> left = mergeSort(arr.sublist(0, mid));
    List<int> right = mergeSort(arr.sublist(mid,));

    arr = [...left, ...right];
    return arr;
  }
  
    static List<int> mergeArray(List<int> arr1,List<int>arr2) {
      List<int> ans = []; 

      while (arr1.isNotEmpty && arr2.isNotEmpty) {
	if (arr1[0] < arr2[0]) {
	  ans.add(arr1.removeAt(0));
      } else {
	  ans.add(arr2.removeAt(0));
      }
      }
      return [...ans, ...arr1, ...arr2];
    }
}
