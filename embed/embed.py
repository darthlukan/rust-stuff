#!/usr/bin/env python3

from ctypes import cdll


lib = cdll.LoadLibrary("target/release/libembed.so")
lib.process()
print("Done!")
