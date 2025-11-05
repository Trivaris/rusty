{
  rustPlatform,
  glib,
  pkg-config
}:
rustPlatform.buildRustPackage {
  name = "rusty";
  src = ./.;
  buildInputs = [ glib ];
  nativeBuildInputs = [ pkg-config ];
  cargoLock.lockFile = ./Cargo.lock;
}