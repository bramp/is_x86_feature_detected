#[macro_export]
macro_rules! are_x86_features_detected {
    ( $( $x:tt ),* ) => {
        {
            $(
                if is_x86_feature_detected!($x) {
                    println!("\u{001b}[32m{:20}: Supported\u{001b}[0m", $x);
                } else {
                    println!("\u{001b}[31m{:20}: Not Supported\u{001b}[0m", $x);
                };
            )*
        }
    };
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn main() {
    println!("Only works on x86");
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn main() {
    are_x86_features_detected![
        "aes",
        "pclmulqdq",
        "rdrand",
        "rdseed",
        "tsc",
        "mmx",
        "sse",
        "sse2",
        "sse3",
        "ssse3",
        "sse4.1",
        "sse4.2",
        "sse4a",
        "sha",
        "avx",
        "avx2",
        "avx512f",
        "avx512cd",
        "avx512er",
        "avx512pf",
        "avx512bw",
        "avx512dq",
        "avx512vl",
        "avx512ifma",
        "avx512vbmi",
        "avx512vpopcntdq",
        "avx512vbmi2",
        "avx512gfni",
        "avx512vaes",
        "avx512vpclmulqdq",
        "avx512vnni",
        "avx512bitalg",
        "avx512bf16",
        "avx512vp2intersect",
        "f16c",
        "fma",
        "bmi1",
        "bmi2",
        "abm",
        "lzcnt",
        "tbm",
        "popcnt",
        "fxsr",
        "xsave",
        "xsaveopt",
        "xsaves",
        "xsavec",
        "cmpxchg16b",
        "adx",
        "rtm"
    ];
}
