export function sumOfIntervals(
  intervals: [start: number, end: number][]
): number {
  let ans = new Set();

  intervals.map((range) => {
    let arr = Array.from(
      { length: range[1] - range[0] },
      (_, k) => k + range[0]
    );
    arr.map((x) => ans.add(x));
  });

  return ans.size;
}
type Type<T extends (arg0: any) => void> = (
  Parameters: Parameters<T>[0]
) => ReturnType<T>;
function maintests(func: Type<typeof sumOfIntervals>) {
  console.log(
    "\t\t\t----TESTS 🧪----",
    "\n✔️ 1st: ",
    func([[1, 5]]) === 4,
    "\n✔️ 2nd: ",
    func([
      [1, 5],
      [6, 10],
    ]) === 8,

    "\n✔️ 3rd: ",
    func([
      [1, 5],
      [1, 5],
    ]) === 4,
    "\n✔️ 4th: ",
    func([
      [1, 4],
      [7, 10],
      [3, 5],
    ]) === 7,
    "\n\t\t----Finished----"
  );
}
maintests(sumOfIntervals);
