#[pkg(id = "http_client",
      vers = "0.1.0")];

extern mod rustpkg;
use core::run;

#[pkg_do(build)]
fn main() {
    run::run_program(~"./configure", []);
    run::run_program(~"make", [~"libhttp_parser.a"]);
    let crate = rustpkg::Crate(~"http_client.rc");
    rustpkg::build(~[crate]);
}
