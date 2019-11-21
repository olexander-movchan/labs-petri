use labs_petri::PTNet;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    task3();
}

fn task3() {
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

fn task4() {
    let mut net = PTNet::new();

    let prod = net.place("prod", 0);
    let cons = net.place("cons", 0);

    let rand = net.place("rand", 0);

    net.transition("-> prod").output(prod.clone(), 1);
    net.transition("-> cons").output(cons.clone(), 1);
    net.transition("-> rand").output(rand.clone(), 1);

    let buff_free = net.place("buff_free", 100);
    let buff_used = net.place("buff_used", 0);

    net.transition("prod, buff_free -> buff_used")
        .input(prod.clone(), 1)
        .input(rand.clone(), 1)
        .input(buff_free.clone(), 1)
        .output(buff_used.clone(), 1);

    net.transition("cons, buff_used -> buff_cond")
        .input(cons.clone(), 1)
        .input(rand.clone(), 1)
        .input(buff_used.clone(), 1)
        .output(buff_free.clone(), 1);

    let avg_buff = &mut 0;
    let iters = net.run(100, || {
        *avg_buff += buff_used.borrow().tokens;
    });

    net.log_state();
    println!("Average buffer usage: {}", *avg_buff as f32 / iters as f32);
}

fn task5() {
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
