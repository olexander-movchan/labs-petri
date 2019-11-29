use labs_petri::PTNet;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut net = PTNet::new();

    let res = net.place("res", 6);

    let full = net.place("full", 0);
    let half = net.place("half", 0);
    let third = net.place("third", 0);

    let full_counter = net.place("full_counter", 0);
    let half_counter = net.place("half_counter", 0);
    let third_counter = net.place("third_counter", 0);

    net.transition("-> full").output(full.clone(), 1);
    net.transition("-> half").output(half.clone(), 2);
    net.transition("-> third").output(third.clone(), 3);

    net.transition("full -> full_counter")
        .input(full.clone(), 1)
        .input(res.clone(), 6)
        .output(full_counter.clone(), 1)
        .output(res.clone(), 6);

    net.transition("half -> half_counter")
        .input(half.clone(), 1)
        .input(res.clone(), 3)
        .output(half_counter.clone(), 1)
        .output(res.clone(), 3);

    net.transition("third -> third_counter")
        .input(third.clone(), 1)
        .input(res.clone(), 2)
        .output(third_counter.clone(), 1)
        .output(res.clone(), 2);

    net.run(100, || {});

    net.log_state();
}
