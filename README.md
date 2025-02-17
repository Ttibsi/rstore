# rstore

A simple key-value store inspired by redis, written as an experiment.
Featuring JSON exporting and a server TUI to see the current contents of the data store

![](https://github.com/user-attachments/assets/4925208e-0a17-4853-9516-22a4623cfd87)

### How to use
Use `netcat` to push commands to the running server, ex:

```console
$ echo -n "ADD names LIST|ADD names ttibsi|SHOW|REM names 0|SHOW" | netcat localhost 9876
```

Valid commands: ADD, DEL, SHOW, HELP, EXPORT

* ADD K V - set key K to value V. if K holds a list, append the value
* ADD K - set key K to True
* ADD K LIST - Set key K to an empty array
* SHOW - Show the contents of the store
* SHOW K - Show the value of key K
* DEL K - Delete key K
* DEL K I - if key K contains a list, remove element at index I
* EXPORT - print a JSON string form of the current contents to stdout.

Commands can be chained with a pipe character `|`
