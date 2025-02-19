use std::env;

// args from the basis cmake file
fn build_with_common_settings() -> cc::Build {
    let mut build = cc::Build::new();

    if !build.get_compiler().is_like_msvc() {
        build
            .flag_if_supported("-fvisibility=hidden")
            .flag_if_supported("-fno-strict-aliasing")
            .flag_if_supported("-Wno-everything");
    } else {
        build.flag_if_supported("/w");
    }

    build
}

fn main() {
    let mut build = build_with_common_settings();

    if !build.get_compiler().is_like_msvc() {
        build.std("c++11");
    }

    if env::var("TARGET")
        .map(|target| target.starts_with("x86_64"))
        .unwrap_or_default()
    {
        build.define("BASISU_SUPPORT_SSE", "1");
        if build.get_compiler().is_like_msvc() {
            build.flag_if_supported("/arch:AVX");
        } else {
            build.flag_if_supported("-msse4.2");
        }
    } else {
        build.define("BASISU_SUPPORT_SSE", "0");
    }

    build
        .cpp(true)
        .file("vendor/basis_universal/encoder/pvpngreader.cpp")
        .file("vendor/basis_universal/encoder/jpgd.cpp")
        .file("vendor/basis_universal/encoder/basisu_uastc_enc.cpp")
        .file("vendor/basis_universal/encoder/basisu_ssim.cpp")
        .file("vendor/basis_universal/encoder/basisu_resampler.cpp")
        .file("vendor/basis_universal/encoder/basisu_resample_filters.cpp")
        .file("vendor/basis_universal/encoder/basisu_pvrtc1_4.cpp")
        .file("vendor/basis_universal/encoder/basisu_opencl.cpp")
        .file("vendor/basis_universal/encoder/basisu_kernels_sse.cpp")
        .file("vendor/basis_universal/encoder/basisu_gpu_texture.cpp")
        .file("vendor/basis_universal/encoder/basisu_frontend.cpp")
        .file("vendor/basis_universal/encoder/basisu_etc.cpp")
        .file("vendor/basis_universal/encoder/basisu_enc.cpp")
        .file("vendor/basis_universal/encoder/basisu_comp.cpp")
        .file("vendor/basis_universal/encoder/basisu_bc7enc.cpp")
        .file("vendor/basis_universal/encoder/basisu_basis_file.cpp")
        .file("vendor/basis_universal/encoder/basisu_backend.cpp")
        .file("vendor/basis_universal/transcoder/basisu_transcoder.cpp")
        .file("vendor/basis_universal/zstd/zstd.c")
        .file("vendor/transcoding_wrapper.cpp")
        .file("vendor/encoding_wrapper.cpp")
        .compile("basisuniversal");

    // We regenerate binding code and check it in. (See generate_bindings.sh)
}
