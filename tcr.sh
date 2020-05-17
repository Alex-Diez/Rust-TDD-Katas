cargo test --package $1 --lib day_$2::tests &&
  git commit -am "$3" ||
  git reset --hard
