use labs_petri::PTNet;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut net = PTNet::new();

    let a = net.place("a");
    let b = net.place("b");

    net.transition("-> a")
        .output(a.clone(), 1);

    net.transition("a -> b")
        .input(a, 1)
        .output(b, 2);

    net.run(100);

    net.log_state();
}
