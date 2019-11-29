use labs_petri::PTNet;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut net = PTNet::new();

    let a_ack = net.place("a_ack", 0);
    let a_snd = net.place("a_snd", 0);
    let a_rec = net.place("a_rec", 0);

    let b_ack = net.place("b_ack", 0);
    let b_snd = net.place("b_snd", 0);
    let b_rec = net.place("b_rec", 0);

    let clock = net.place("clock", 1);

    net.transition("-> a_snd")
        .output(a_snd.clone(), 1);

    net.transition("-> b_snd")
        .output(b_snd.clone(), 1);

    net.transition("a_snd, clock -> b_rec, clock")
        .input(a_snd.clone(), 1)
        .input(clock.clone(), 1)
        .output(b_rec.clone(), 1)
        .output(clock.clone(), 1);

    net.transition("a_rec, clock -> b_ack, clock")
        .input(a_rec.clone(), 1)
        .input(clock.clone(), 1)
        .output(b_ack.clone(), 1)
        .output(clock.clone(), 1);


    net.transition("b_snd, clock -> a_rec, clock")
        .input(b_snd.clone(), 1)
        .input(clock.clone(), 1)
        .output(a_rec.clone(), 1)
        .output(clock.clone(), 1);

    net.transition("b_rec, clock -> a_ack, clock")
        .input(b_rec.clone(), 1)
        .input(clock.clone(), 1)
        .output(a_ack.clone(), 1)
        .output(clock.clone(), 1);

    net.run(100, || {});

    net.log_state();
}
