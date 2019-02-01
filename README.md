# cheetah_bf
Brainf*ck interpreter written in Rust.

## Benchmark
Input code: Mandelbrot set generator.  
Version: 1.0.0
CPU: Core i3 3220  
OS: Ubuntu16.04(Bash on Windows)  

|program|time|
|------|------|
|chetah_bf(release build)|0m6.686s|
|cheeta_bf(optimized build)※1|0m5.652s|
|bf|0m7.604s|
|bff|0m5.330s|

※1 cargo rustc --release -- -C lto -C debug_assertions=no -C panic=abort
