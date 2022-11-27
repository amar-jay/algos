import path from 'path'

const filePath = (filename:string) => {
    return path.join(__dirname, filename)
}


let x = filePath('../hello.txt')
console.log(x)
