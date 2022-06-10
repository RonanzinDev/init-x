function main(name) {
    console.log("Hello, World " + name)
}
let name = process.argv.slice(2)
main(name[ 1 ])