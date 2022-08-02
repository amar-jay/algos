function pathFinder(maze:string): boolean{
  const WALL = 'W'
  const PATH = '.'
  let table = maze.split('\n').map(row => row.split(''))
  let max_len = table[0].length - 1
  let stack:[x:number, y:number][] = []
  stack.push([0,0])
  
  while (stack.length != 0) {
    let elem = stack.pop()
    
    let x = elem![0]
    let y = elem![1]
   console.log(stack, x, y, max_len, 'right:' ,table[x][y+1], 'curr:', table[x][y], 'left:', table[x][y-1]) 
    if (x === max_len && y === max_len) {
      return true
    }
    
    if (table[x][y] === WALL){
      continue
    }
  
    //check right
    if (max_len >= y+1 && max_len >= x && table[x][y+1] === PATH){
      stack.unshift([x, y+1])
    }
  
  // check down
   if (max_len >= y && max_len >= x+1 && table[x+1][y] === PATH){
      stack.unshift([x+1, y])
    }
  // check left
   if (max_len >= y-1 && max_len >= x && table[x][y-1] && table[x][y-1] === PATH && table[x][y-1] !== WALL){
      stack.unshift([x, y-1])
    }
  // check up
   if (max_len >= y && max_len >= x-1 && table[x-1][y] && table[x-1][y] === PATH && table[x-1][y] !== WALL){
      stack.unshift([x-1, y])
    }
    
}
  return false;
}

