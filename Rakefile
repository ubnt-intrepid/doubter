require 'rake'

task :nightly_test do
    sh "cargo test --verbose -p doctest-nightly"
end

task :test do
    sh "cargo test --verbose"
    sh "cargo test --verbose -p doubter-impl"
    sh "cargo test --verbose -p doctest"
end

task :install_hooks do
    sh "cargo clean -p cargo-husky"
    sh "cargo check -p cargo-husky"
end

task default: :test
