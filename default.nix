{rustPlatform}:
rustPlatform.buildRustPackage {
  pname = "systemyml";
  version = "0.0.1";

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
