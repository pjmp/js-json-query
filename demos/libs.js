function len(obj) {
    return obj.length
};

function max(arr) {
    return Math.max(...arr)
};

function uniq(arr) {
    return arr.filter((curr, index, self) => self.indexOf(curr) === index)
}

function toString(arr) {
    return arr.join("\n")
}