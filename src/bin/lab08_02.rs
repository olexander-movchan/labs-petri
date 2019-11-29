use labs_petri::PTNet;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

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
