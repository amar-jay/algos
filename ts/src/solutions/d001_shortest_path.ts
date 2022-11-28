
enum CharType {}
type Coord = [x:number, y:number];
type Char = string & CharType;
const my_sol = (maze:string) => {
  let maz:string[]= [];


  const moveLeft = (curr_pos:Coord) => [curr_pos[0]-1, curr_pos[1]] as Coord;
  const moveRight = (curr_pos:Coord) => [curr_pos[0]+1, curr_pos[1]] as Coord;
  const moveDown = (curr_pos:Coord) => [curr_pos[0], curr_pos[1]-1] as Coord ;
  const moveUp = (curr_pos:Coord) => [curr_pos[0], curr_pos[1]+1] as Coord;

  const checkLeft = (curr_pos:Coord) => maz[curr_pos[0]-1][curr_pos[1]];
  const checkRight = (curr_pos:Coord)=> maz[curr_pos[0]+1][curr_pos[1]];
  const checkDown = (curr_pos:Coord) => maz[curr_pos[0]][curr_pos[1]-1];
  const checkUp = (curr_pos:Coord) => maz[curr_pos[0]][curr_pos[1]+1];

  // convert maze from string to 2D
  let tmp = '';
  for (let i of maze) {
    tmp.concat(i);
    if (i === '\n'){
      maz.push(tmp);
      tmp='';
    }
  }

  if (maze.length < 1) {
    return 0;
  }


  /*
  let curr_pos:Coord = [0, 0];
  while (true) {
    switch ('.'){
      case checkLeft(curr_pos):

      break;
      case checkRight(curr_pos):
      break;
      case checkUp(curr_pos):
      break;
      case checkDown(curr_pos):
      break;
	default:
      return false;
    }
    
}
  */

  return maze.length < 1;
}


function aliter(maze:string) {
  const matrix = maze.split(`\n`).map(row => row.split(``));
  const queue = [{ x: 0, y: 0, len: 0 }];
  type Coord = typeof queue[0];
  const n = matrix.length - 1;

  while (queue.length > 0) {
    const { x, y, len }:Coord = queue.shift()!;
    if (x == n && y == n) {
      return len;
    }
    matrix[x][y] = 'W';
    [[x + 1, y], [x - 1, y], [x, y + 1], [x, y - 1]].forEach(([t1, t2]) => {
      if (matrix[t1] && matrix[t1][t2] && matrix[t1][t2] != 'W') {
        queue.push({x: t1, y: t2, len: len + 1});
        matrix[t1][t2] = 'W';
      }
    });
  }
  return false;
}
export default (maze:string) => aliter(maze)
