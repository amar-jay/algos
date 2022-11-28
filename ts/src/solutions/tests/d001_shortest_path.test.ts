import shortest_path from "../d001_shortest_path";

  function testMaze(expected:number | boolean, maze:string){
  let actual = shortest_path(maze);
  expect(actual).toBe(expected);
}

describe("Testing d001_shortest_path:", () => {

  testMaze(4,
  `.W.
  .W.
  ...`);

  testMaze(false,
  `.W.
  .W.
  W..`);

  testMaze(10,
  `......
  ......
  ......
  ......
  ......
  ......`);

  testMaze(false,
  `......
  ......
  ......
  ......
  .....W
  ....W.`);

});
