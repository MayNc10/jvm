#!/bin/bash
cd classes
cargo r -- -r -db $1.class
cd ..   