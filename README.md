# is_x86_feature_detected

by [Andrew Brampton](https://bramp.net)

Very simple app that checks for every CPU feature that Rust supports, and printed the results out.

```shell
bramp@macbook:~/is_x86_feature_detected $ cargo run
   Compiling is_x86_feature_detected v0.1.0 (~/is_x86_feature_detected)
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/is_x86_feature_detected`

aes                 : Supported
pclmulqdq           : Supported
rdrand              : Supported
rdseed              : Supported
tsc                 : Supported
mmx                 : Supported
sse                 : Supported
sse2                : Supported
sse3                : Supported
ssse3               : Supported
sse4.1              : Supported
sse4.2              : Supported
sse4a               : Not Supported
sha                 : Not Supported
avx                 : Supported
avx2                : Supported
avx512f             : Not Supported
avx512cd            : Not Supported
avx512er            : Not Supported
avx512pf            : Not Supported
avx512bw            : Not Supported
avx512dq            : Not Supported
avx512vl            : Not Supported
avx512ifma          : Not Supported
avx512vbmi          : Not Supported
avx512vpopcntdq     : Not Supported
avx512vbmi2         : Not Supported
avx512gfni          : Not Supported
avx512vaes          : Not Supported
avx512vpclmulqdq    : Not Supported
avx512vnni          : Not Supported
avx512bitalg        : Not Supported
avx512bf16          : Not Supported
avx512vp2intersect  : Not Supported
f16c                : Supported
fma                 : Supported
bmi1                : Supported
bmi2                : Supported
abm                 : Supported
lzcnt               : Supported
tbm                 : Not Supported
popcnt              : Supported
fxsr                : Supported
xsave               : Supported
xsaveopt            : Supported
xsaves              : Supported
xsavec              : Supported
cmpxchg16b          : Supported
adx                 : Supported
rtm                 : Supported
```

## Other useful commands

```

$ sysctl -n machdep.cpu.brand_string
Intel(R) Core(TM) i9-8950HK CPU @ 2.90GHz

$ sysctl -a | grep cpu.features
machdep.cpu.features: FPU VME DE PSE TSC MSR PAE MCE CX8 APIC SEP MTRR PGE MCA CMOV PAT PSE36 CLFSH DS ACPI MMX FXSR SSE SSE2 SS HTT TM PBE SSE3 PCLMULQDQ DTES64 MON DSCPL VMX EST TM2 SSSE3 FMA CX16 TPR PDCM SSE4.1 SSE4.2 x2APIC MOVBE POPCNT AES PCID XSAVE OSXSAVE SEGLIM64 TSCTMR AVX1.0 RDRAND F16C
```


```
Copyright 2022 Andrew Brampton

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```