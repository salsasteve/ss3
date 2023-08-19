
import sys
with open(sys.argv[1], 'rb') as f:
    for c in f.read():
        print(bin(c)[2:])