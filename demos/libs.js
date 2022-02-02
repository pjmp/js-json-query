function len(obj) {
    return obj.length
};

function max(arr) {
    return Math.max(...arr)
};

function uniq(arr) {
    return arr.filter((curr, index, self) => self.indexOf(curr) === index)
}

function pick(obj, ...keys) {
    return keys.reduce((init, curr) => {
        init[curr] = obj[curr];
        return init
    }, {})
}

function has(key, obj = it) {
    return key in obj
}

function keys(obj = it) {
    return Object.keys(obj)
}

function join(arr, sep = `\n`) {
    return arr.join(sep)
}

function head(array) {
    return array[0]
}

function tail(array) {
    return array[array.length - 1]
}

function summary(data = it) {
    function type(data) {
        var t = Object.prototype.toString.call(data);
        var t = t.slice(8, t.length - 1);

        return t;
    }

    return Object.keys(data).reduce((init, item) => {
        init[item] = `<${type(data[item])}>`
        return init;
    }, {})
}
