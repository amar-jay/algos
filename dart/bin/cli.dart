import 'package:cli/algos.dart';

void main(List<String> arguments) {
  List<int> nums = [1, 2, 4 , 6];
  if (Algos.binarySearch(nums, 4) != 2) {
    print("Test Failed: Binary search");
    return;
  } 

  List<int> l = [9, 8, 9, 3];
  List<int> r = [1, 2, 3, 4];
  List<int> lr = [1, 1, 2, 2, 3, 4, 4, 6];
  //List<int> a = [...l, ...r];
  List<int> an = Algos.mergeArray(r, l);
  print("$lr $an");

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
  print("Tests Passed");

}
