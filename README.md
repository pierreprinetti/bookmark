# bookmark

Save a URL for later.

## Requirements

* bash
* jq

## Use
List bookmarks:
```shell
./bookmark
```

Add a new bookmark:
```shell
./bookmark <url> <description>
```

Print the database filename:
```shell
./bookmark -F
```

This help:
```shell
./bookmark --help
```

Print the version string:
```shell
./bookmark --version
```

## Example

```shell
$ ./bookmark -t 'opensource' https://opensource.com 'Tips, tricks and news about opensource'
```
