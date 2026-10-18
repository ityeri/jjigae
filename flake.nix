{
  description = "jjigae dev shell (wayland/vulkan runtime libs for winit + wgpu)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        runtimeLibs = with pkgs; [
          # winit wayland backend
          wayland
          wayland-protocols
          libxkbcommon

          # winit x11 backend (fallback)
          libx11
          libxcursor
          libxrandr
          libxi

          # wgpu vulkan backend
          vulkan-loader
          vulkan-validation-layers

          # GL fallback
          libGL
        ];
      in
      {
        devShells.default = pkgs.mkShellNoCC {
          # mkShellNoCC를 쓰는 이유: 일반 mkShell은 이 flake가 고정한
          # nixpkgs의 gcc(+glibc)를 PATH에 주입한다. 그 glibc가 시스템(및
          # rustup nightly 툴체인)이 쓰는 glibc보다 버전이 달라지면,
          # spirv-builder가 빌드하는 rustc_codegen_spirv.so가 더 높은 버전의
          # glibc 심볼을 요구하게 되어 "GLIBC_x.xx version not found" 런타임
          # 에러가 난다. C 컴파일러는 시스템 PATH(cc)의 것을 그대로 쓰도록
          # 비워둔다.
          buildInputs =
            runtimeLibs
            ++ (with pkgs; [
              pkg-config
            ]);

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
        };
      }
    );
}
