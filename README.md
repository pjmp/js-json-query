# jsjq

> A tool for processing JSON inputs with JavaScript, no dsl!

# Intro

[jq](https://stedolan.github.io/jq/) is an awesome tool if you are well versed in it go ahead, use it.

I don't use `jq` all the time and consequently I don't remember it's [filter](https://stedolan.github.io/jq/manual/#Basicfilters) DSL syntax, however I do know JavaScript so it's easier and quicker for me to fire node and do whatever I want.

`jsjq` is a convenient way to combine usability of `jq` and scripting ability of nodejs.

By default if no code is passed, `jsjq` will pretty print the input json to stdout.

The input json can be accessed in the script with the variable name `it`.

# Example

```shell
jsjq 'it.feeds[0].multiMedia[0]' -f jsonfilewithhierarchy-100-100.json

echo '{"key": "value"}' | jsjq

jsjq 'it[0]' < EmployeeData.json

cat EmployeeData.json | jsjq 'it.map(t => ({name: t.name, age: t.email, liveLocation: t.liveLocation}))'

jsjq 'let p = it.map(t => t.password); max(p.map(len))' -i demos/libs.js -f EmployeeData.json
```

# Limitations

- The code passed should be a valid JavaScript code as this uses v8 engine to run the script therefore sometimes it can get quite verbose.

- The script passed should end with an expression not statement, if statement ends the script then `undefined` will be printed.

Example:

```shell
jsjq 'let t = it.key;' --file your-json-file.json # output -> undefined

jsjq 'let t = it.key; t' --file your-json-file.json # output json -> {...}
```

- The script passed is stringified with double quotes `"` and evaluted using `eval` so don't use `"` in your script, instead use single quote `'` or back tick `

Example:

```shell
# fails with `SyntaxError`
jsjq 'it["someKey"]' < json_file.json

# Ok
jsjq 'it[`someKey`]' < json_file.json
```
