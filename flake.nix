{
inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
};

outputs = { self, nixpkgs }:
    let
        system = "x86_64-linux";
        pkgs = import nixpkgs {
            inherit system;
        };

        commonPackages = [
            #c/c++
            pkgs.cmake
            pkgs.gcc
            pkgs.clang
            pkgs.pkg-config

            pkgs.glfw

            #opengl
            pkgs.libGL

            #wayland
            pkgs.wayland
            pkgs.libxkbcommon

            #x11
            pkgs.libx11
            pkgs.libxext
            pkgs.libxcursor
            pkgs.libxrandr
            pkgs.libxi
            pkgs.libXinerama

            #rust
            pkgs.cargo
            pkgs.rustc
            pkgs.rustfmt
            pkgs.clippy
        ];
    in {
        devShells.${system} = {
            default = pkgs.mkShell {
                packages = commonPackages;

                LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
                    pkgs.glfw
                    pkgs.libGL
                    pkgs.wayland
                    pkgs.libxkbcommon
                    pkgs.libx11
                    pkgs.libxext
                    pkgs.libxcursor
                    pkgs.libxrandr
                    pkgs.libxi
                    pkgs.libXinerama
                ];

                LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

                shellHook = ''

                '';
            };

            ide = pkgs.mkShell {
                packages = commonPackages ++
                    [(pkgs.vscode-with-extensions.override {
                        vscode = pkgs.vscodium;
                        vscodeExtensions = with pkgs.vscode-extensions; [
                            rust-lang.rust-analyzer
                            vadimcn.vscode-lldb
                        ] ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [
                            {
                            name = "shader";
                            publisher = "slevesque";
                            version = "1.1.5";
                            sha256 = "sha256-Pf37FeQMNlv74f7LMz9+CKscF6UjTZ7ZpcaZFKtX2ZM=";
                            }
                        ];
                    })];

                LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
                    pkgs.glfw
                    pkgs.libGL
                    pkgs.wayland
                    pkgs.libxkbcommon
                    pkgs.libx11
                    pkgs.libxext
                    pkgs.libxcursor
                    pkgs.libxrandr
                    pkgs.libxi
                    pkgs.libXinerama
                ];

                LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

                shellHook = ''

                '';
            };
        };
    };
}
