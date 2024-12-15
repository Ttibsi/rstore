# rstore

A simple key-value store that probably doesn't work great, but kinda does.
It's mostly an experiment.

### How to use
Use `netcat` to push commands to the running server, ex:

```console
$ echo -n "PUSH foo:bar|VIEW|REMOVE 0|VIEW" | netcat localhost 9876
```

Valid commands:
* PUSH - Push an item into the current structure
* REMOVE - Remove an item from the current data store
* VIEW - Print out the current stored data structure
* ECHO - print out the parts of the command entered, used for debugging

Any printing commands will show on the server, not return over the network
Commands can be chained with a pipe character `|`
