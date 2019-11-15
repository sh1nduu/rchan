Rust implementation of [compilerbook](https://www.sigbus.info/compilerbook)  

## Docker Setup

```
$ docker build -t compilerbook .
```

## Execute

```
$ docker run --rm -v $(pwd):/rchan -w /rchan compilerbook /bin/bash test.sh
```