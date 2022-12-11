import 'package:cli/algos.dart';

void main(List<String> arguments) {
  List<int> nums = [1, 2, 4 , 6];
  if (Algos.binarySearch(nums, 4) == 2) {
    print("Test Passed: Binary search");
    return;
  } 

  List<int> lw = [6, 1, 4, 2];
  List<int> lm = [1, 2, 4, 6];
  if (Algos.mergeSort(lw) == lm) {
    print("Test Passed: Merge sort");
    return;
  } 
  print("Test failed");

}
