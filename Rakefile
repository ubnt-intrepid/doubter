require 'rake'

task :nightly_test do
    sh "cargo test -p doctest-nightly"
end

task :edition2018_test do
    sh "cargo test -p doctest-edition2018"
end

task :test do
    sh "cargo test"
    sh "cargo test -p doubter-impl"
    sh "cargo test -p doctest"
    sh "cargo test -p doctest-extract"
    sh "cargo test -p doctest-script"
end

task :install_hooks do
    sh "cargo clean -p cargo-husky"
    sh "cargo check -p cargo-husky"
end

task default: :test
