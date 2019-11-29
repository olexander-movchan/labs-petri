use labs_petri::PTNet;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut net = PTNet::new();

    let a = net.place("a", 100);
    let b = net.place("b", 0);
    let c = net.place("c", 0);

    net.transition("a -> b")
        .input(a.clone(), 1)
        .output(b.clone(), 1);

    net.transition("a -> c")
        .input(a.clone(), 1)
        .output(c.clone(), 1);

    net.run(100, || {});

    net.log_state();
}
