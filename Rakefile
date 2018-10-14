require 'rake'

task :lint do
    sh "cargo fmt -- --check"
    sh "cargo clippy --all-targets -p doubter"
    sh "cargo clippy --all-targets -p doubter-impl"
    sh "cargo clippy --all-targets -p doubter-macros"
end

task :test do
    sh "cargo test -p doubter"
    sh "cargo test -p doubter-impl"
    sh "cargo test -p doubter-macros"
    sh "cargo test -p doctest"
    sh "cargo test -p doctest-extract"
    sh "cargo test -p doctest-script"
end

task :test_edition2018 do
    sh "cargo test -p doctest-edition2018"
end

task :test_nightly do
    sh "cargo test -p doctest-nightly"
end

task :install_hooks do
    sh "cargo clean -p cargo-husky"
    sh "cargo check -p cargo-husky"
end

task default: :test
