import 'package:cli/algos.dart';

void main(List<String> arguments) {
  List<int> nums = [1, 2, 4 , 6];
  if (Algos.binarySearch(nums, 4) != 2) {
    print("Test Failed: Binary search");
    return;
  } 

  List<int> lw = [6, 1, 4, 2];
  List<int> lm = [1, 2, 4, 6];
  List<int> ans = Algos.mergeSort(lw);
  for (int i in ans) {
    for (int j in lm) {
      if (i != j) {
      print("Test Failed: Merge sort");
      }
    print("r:$i w:$j");
    }
  }
  if (ans != lm) {
    print("Test Failed: Merge sort $ans $lm");
    return;
  } 
  print("Test failed");

}
