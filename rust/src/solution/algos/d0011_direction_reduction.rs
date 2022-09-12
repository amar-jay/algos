/**
* Once upon a time, on a way through the old wild mountainous west,…
… a man was given directions to go from one point to another. The directions were "NORTH", "SOUTH", "WEST", "EAST". Clearly "NORTH" and "SOUTH" are opposite, "WEST" and "EAST" too.

Going to one direction and coming back the opposite direction right away is a needless effort. Since this is the wild west, with dreadful weather and not much water, it's important to save yourself some energy, otherwise you might die of thirst!

How I crossed a mountainous desert the smart way.
The directions given to the man are, for example, the following (depending on the language):

["NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST"].
or
{ "NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST" };
or
[North, South, South, East, West, North, West]
You can immediately see that going "NORTH" and immediately "SOUTH" is not reasonable, better stay to the same place! So the task is to give to the man a simplified version of the plan. A better plan in this case is simply:

["WEST"]
or
{ "WEST" }
or
[West]
Other examples:
In ["NORTH", "SOUTH", "EAST", "WEST"], the direction "NORTH" + "SOUTH" is going north and coming back right away.

The path becomes ["EAST", "WEST"], now "EAST" and "WEST" annihilate each other, therefore, the final result is [] (nil in Clojure).

In ["NORTH", "EAST", "WEST", "SOUTH", "WEST", "WEST"], "NORTH" and "SOUTH" are not directly opposite but they become directly opposite after the reduction of "EAST" and "WEST" so the whole path is reducible to ["WEST", "WEST"].

Task
Write a function dirReduc which will take an array of strings and returns an array of strings with the needless directions removed (W<->E or S<->N side by side).

The Haskell version takes a list of directions with data Direction = North | East | West | South.
The Clojure version returns nil when the path is reduced to nothing.
The Rust version takes a slice of enum Direction {North, East, West, South}.
See more examples in "Sample Tests:"
Notes
Not all paths can be made simpler. The path ["NORTH", "WEST", "SOUTH", "EAST"] is not reducible. "NORTH" and "WEST", "WEST" and "SOUTH", "SOUTH" and "EAST" are not directly opposite of each other and can't become such. Hence the result path is itself : ["NORTH", "WEST", "SOUTH", "EAST"].
if you want to translate, please ask before translating.
*
*/

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

use Direction::*;
#[allow(unused)]
fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut ans:Vec<Direction> = vec![];
    let opp = |dir:Direction| match dir {
        North => South,
        South => North,
        East => West,
        West => East
    };

    for dir in arr {
        if !ans.is_empty() && opp(*dir) == *ans.last().unwrap() {
            ans.pop();
        } else {
            ans.push(*dir);
        }
    }
    ans
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);
        
        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }

    
    #[test]
    fn advanced() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);
        
        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
        
        let a = [North, South, South, East, West, North, North];
        assert_eq!(dir_reduc(&a), [North]);
        
        let a = [East, East, West, North, West, East, East, South, North, West];
        assert_eq!(dir_reduc(&a), [East, North]);
        
        let a = [North, East, North, East, West, West, East, East, West, South];
        assert_eq!(dir_reduc(&a), [North, East]);
        
        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }
    use rand::{Rng, thread_rng};
    
    #[test]
    fn random() {
        let mut rng = thread_rng();
        
        for _ in 0..100 {
            let a = (0..rng.gen_range(1..10))
                .map(|_| match rng.gen_range(0..4) {
                    0 => North,
                    1 => East,
                    2 => South,
                    _ => West,
                })
                .collect::<Vec<_>>();
            
            let actual = dir_reduc(&a);
            let expect = dir_reduc_solution(&a);
            
            assert_eq!(
                actual, expect,
                "with directions {:?}",
                a
            );
        }
    }
    
    fn dir_reduc_solution(arr: &[Direction]) -> Vec<Direction> {
        let mut result: Vec<Direction> = Vec::new();
        
        for &s in arr {
            if !result.is_empty() && can_be_reduced(s, *result.last().unwrap()) {
                result.pop();
            } else {
                result.push(s);
            }
        }
        
        result
    }
    
    fn can_be_reduced(elem: Direction, last: Direction) -> bool {
        matches!((elem, last), (North, South) | 
        (South, North) | 
        (West, East) | 
        (East, West))
    }
}
