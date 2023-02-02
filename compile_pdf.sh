#!/usr/bin/env bash

pandoc -s README.md hello-world.md control-flow.md primitives.md unique.md borrowed.md data-types.md rc-raw.md destructuring.md destructuring-2.md arrays.md graphs/README.md closures.md -o r4cppp.pdf
