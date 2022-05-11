# Define mozilla overlay
let
  rust_moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  glibc_overlay = (self: super: { gcc = self.gcc7; });
in
with import <nixpkgs> { overlays = [rust_moz_overlay];};
let
    rust = latest.rustChannels.stable.rust.override {
        extensions = [
            "rust-src"
            # "rls-preview"
            # "rust-analysis"
            "rustfmt-preview"
        ];
    };
    /* vscode-extensions; */
    vscode_extensions = (with pkgs.vscode-extensions; [
      bbenoist.nix
      vscode-extensions.matklad.rust-analyzer
      tamasfe.even-better-toml
      yzhang.markdown-all-in-one
      vadimcn.vscode-lldb
    ]) ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [
      # {
      #   name = "vscode-lldb";
      #   publisher = "vadimcn";
      #   version = "1.7.0";
      #   sha256 = "0sdy261fkccff13i0s6kiykkwisinasxy1n23m0xmw72i9w31rhf";
      # }
    ];
  vscode-with-extensions = pkgs.vscode-with-extensions.override {
      vscodeExtensions = vscode_extensions;
    };
in
mkShell {
  name = "rust-env";
  nativeBuildInputs = with buildPackages; [

    rust
    vscode-with-extensions

    #cross.binutils
    # cross.stdenv.glibc

    /* vscode # visual studop code */

    # latest.rustChannels.nightly.rust.override {extensions = [ "rust-src" ];}
    # rustup
    #pkg-config
    # git

    # rust_latest_override
    # unstable.rust-analyzer
    /* protobuf
    protobuf3_9
    sbt */
    # protobuf3_9


    # rls
    # lldb
  ];

  buildInputs = [
    /* cross.stdenv.cc */
    /* cross.binutils */
  ];

  # This environment variable specifies linker for armv7-unknown-linux-gnueabihf target
  #CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER = "armv7l-unknown-linux-gnueabihf-gcc";
  #CARGO_TARGET_DIR = "/home/misha/NFS/public/cmpt433/baremetal";

  #RUSTFLAGS="-C link-arg=-Wl,-dynamic-linker,/lib/ld-linux-armhf.so.3";
  #RUSTFLAGS = "";

}
