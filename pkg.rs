#[pkg(id = "http-client",
      vers = "0.1.0")];

use core::run;

#[pkg_do(build)]
fn main() {
    run::run_program("./configure", []);
    run::run_program("make", []);
}
