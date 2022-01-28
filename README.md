# js-json-query (jsjq)

> A tool for processing JSON inputs with real JavaScript, no dsl!

# Intro

[jq](https://stedolan.github.io/jq/) is an awesome tool if you are well versed in it go ahead, use it.

I don't use `jq` all the time and consequently I don't remember it's [filter](https://stedolan.github.io/jq/manual/#Basicfilters) DSL syntax, however I do know JavaScript so it's easier and quicker for me to fire node and do whatever I want.

`jsjq` is a convenient way to combine usability of `jq` and scripting ability of nodejs.

# Example

```shell
jsjq 'it.feeds[0].multiMedia[0]' -p jsonfilewithhierarchy-100-100.json

echo '{"key": "value"}' | jsjq 'it'

jsjq 'it[0]' < EmployeeData.json

cat EmployeeData.json | jsjq 'it.map(t => ({name: t.name, age: t.email, liveLocation: t.liveLocation}))'
```
