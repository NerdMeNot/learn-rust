#!/bin/sh
# Prints the assembly of one function from the .s file that
#   cargo rustc [--release] --lib -- --emit asm
# wrote: sh asm.sh release sum_to      (directives that start with a dot are left out)
awk "/^_R.*$2:/,/cfi_endproc/" target/$1/deps/*.s | grep -v '^[[:space:]]*\.'
