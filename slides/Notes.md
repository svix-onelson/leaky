

```
cargo build
BH=$(ls ~/Apps/bytehound/libbytehound.so)                                                        
LD_PRELOAD=$BH target/debug/leaky
```

Let that run for a while...

```
du -h *.dat
```

```
bytehound server *.dat
```

## Demo

(Talk through main.rs)

---

(Build, run the program, watch htop briefly)

---

Classifications for memory:

- SHR, non-volatile stuff loaded from the binary/libs.
- RES, aka RSS? Growable. Heap usage tracks here.
- VIRT - no idea. Seems highly speculative. I tend to ignore this.

(Fire up the GUI)

https://gist.github.com/svix-onelson/18c268a628c611dc9921ed04ee1deef3