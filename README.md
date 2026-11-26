#### bonsai - a rudimentary tree clone  
https://en.wikipedia.org/wiki/Tree_(command)  
```shell
$ bonsai --help
Usage: bonsai [path]
$ bonsai brfk
.brfk
├─── internal
│    ├─── tokenizer
│    │    ├─── tokenizer.go
│    │    ├─── token.go
│    │    ├─── tokenizer_test.go
│    │    ├─── optimizer_test.go
│    │    └─── optimizer.go
│    ├─── config
│    │    └─── config.go
│    ├─── utils
│    │    └─── utils.go
│    └─── runner
│         ├─── runner.go
│         ├─── handlers.go
│         └─── program.go
├─── cover.png
├─── .gitattributes
├─── Makefile
├─── cmd
│    └─── main.go
├─── LICENSE
├─── .gitignore
├─── go.mod
├─── README.md
└─── bfcode
     ├─── helloworld.bf
     ├─── beer.bf
     ├─── echo.bf
     ├─── mandelbrot.bf
     ├─── simple.bf
     └─── clearloop.bf
```
currently it does not support excluding gitignored and hidden files.
