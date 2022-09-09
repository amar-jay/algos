/**
 * create a basic react todo list
 * @param {string} title - title of the todo
 * @returns {{date:string, title:string}} todo object
 * @example
 * createTodo('buy milk') // {date: '2019-01-01', title: 'buy milk'}
 */

function createTodo(title: string) {
  return {
    date: new Date().toISOString(),
    title,
  };
}

/**
 * create tests for createTodo function
 */

function tests() {
  const todo = createTodo("buy milk");
  const ans = compareTypes(todo, { date: "", title: "buy milk" });
  console.log("tests: ", ans, ans ? "💡" : "💥");
  // check if the todo is the same type as the createTodo function
}

tests();

function compareTypes(
  a: ReturnType<typeof createTodo>,
  b: ReturnType<typeof createTodo>
) {
  //compare types of a and b
  // TODO: find a way to compare types of a and b
  return false;
}
