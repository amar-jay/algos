function createTodo(title) {
    return {
        date: new Date().toISOString(),
        title,
    };
}
function tests() {
    const todo = createTodo("buy milk");
    const ans = compareTypes(todo, { date: "", title: "buy milk" });
    console.log("tests: ", ans, ans ? "💡" : "💥");
}
tests();
function compareTypes(a, b) {
    return false;
}
//# sourceMappingURL=findPrimes.js.map