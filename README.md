# Langbot

## Creating an image

1. Install podman and make
2. Build an image: `make build`

## Running a language

Run `make run L=<some language>`. It will read source code
from stdin and execute it.

## Adding your own language

1. Create a directory `langs/yourlang/`.
2. Create a file `langs/yourlang/compile.sh`.
	* This file is responsible for downloading sources for, compiling and installing
	  your programming language.
	* It will be run using the command `bash -x -euo pipefail compile.sh`,
	  in a temporary working directory which is populated with the content of `langs/yourlang`.
	* When downloading sources, always make sure that what you're downloading won't change.
	  E.g with git, always `git checkout` a particular commit.
	* Install all build artifacts to the directory that's passed in as
	  the `$DEPLOYDIR` environment variable.
	* When everything is done, run `touch "$DEPLOYDIR/.done"` as a final sanity check.
3. Create a file `langs/yourlang/run.sh`.
	* `run.sh` is copied to the deployment directory (what was `$DEPLOYDIR` in `compile.sh`)
	* It will be run using the command `bash -euo pipefail run.sh`,
	  in the deployment directory. It should read its source code from stdin,
	  and do whatever is necessary to execute that code.
	* If everything goes okay, exit with exit code 0; if something goes wrong,
	  exit with a non-zero exit code.
	* You can assume that only one execution happens at a time, meaning you shouldn't worry
	  about creating random file names for the input files.
4. Add your language to the list in `scripts/compile-all.sh` to make it run
   as part of building the podman container.

In some cases, you may also need to add dependencies to the `Containerfile`.

A simple example language you can take inspiration from is Osyris.

`langs/osyris/compile.sh` is:

```shell
git clone https://github.com/mortie/osyris.git
cd osyris
git checkout 2db1f2c1746dbda2451d8cc888921a0f518aaf78

cargo build --release
cp target/release/osyris "$DEPLOYDIR"

touch "$DEPLOYDIR/.done"
```

And `langs/osyris/run.sh` is:

```
cat >input.os
exec ./osyris input.os
```

### Some development tips

* During development, it might help to replace the `RUN ./scripts/compile-all.sh`
  line in the `Containerfile` with `RUN ./scripts/compile.sh yourlang>`,
  to avoid having to build all languages.
* Don't be afraid to re-build the container; podman caches all the steps it can,
  so `make build` doesn't start from scratch every time.
* You may want to investigate the compiler interactively, which can be done with
  `make shell`.
* In fact, I find it helpful to manually run my shell commands in an interactive
  container and write the `compile.sh` script by writing down my commands there
  as I go.
