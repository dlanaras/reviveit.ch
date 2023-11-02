{
  rustPlatform,
  sqlite,
}:
rustPlatform.buildRustPackage {
  name = "backend";

  src = ./backend;
  cargoLock.lockFile = backend/Cargo.lock;

  buildInputs = [
    sqlite
  ];
}
