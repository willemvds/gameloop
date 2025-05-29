use std::cmp;
use std::thread;
use std::time;

fn gather_input_events() {
}

fn simulation(prev_sim_at: time::Instant, dt: u64) {
    let now = time::Instant::now();
    let mut sims = 0;
    let mut remainder = now.duration_since(prev_sim_at).as_micros() as u64;
    while remainder > dt {
        sims += 1;
        remainder -= dt;

        // pretend to work
        thread::sleep(time::Duration::from_micros(500));
    }
    eprintln!("Simulation at {:?}, sims={}", time::Instant::now(), sims);
}

fn render() {
    // pretend to render
    thread::sleep(time::Duration::from_micros(16_000));
    eprintln!("Render at {:?}", time::Instant::now());
}

fn main() {
    let tick_rate = 4_000; //microseconds - 250Hz
    let frame_rate = 8_333; //microseconds - 120Hz

    let mut prev_simulation_tick_at = time::Instant::now();
    let mut prev_render_frame_at = time::Instant::now();

    loop {
        let next_simulation_tick_at =
            prev_simulation_tick_at + time::Duration::from_micros(tick_rate);
        let next_render_frame_at = prev_render_frame_at + time::Duration::from_micros(frame_rate);

        let next_event_at = cmp::min(next_simulation_tick_at, next_render_frame_at);
        let now = time::Instant::now();
        let sleep_duration = next_event_at - now;

        if sleep_duration.as_micros() > 0 {
            let sleep_started_at = now;
            // replace this with the "WaitMachine" concept
            thread::sleep(sleep_duration);
            let sleep_completed_at = time::Instant::now();
            eprintln!(
                "Done with sleep, wanted={}, got={}",
                sleep_duration.as_micros(),
                sleep_completed_at
                    .duration_since(sleep_started_at)
                    .as_micros(),
            );
        }

        let realnow = time::Instant::now();
        if realnow > next_simulation_tick_at {
            let x = prev_simulation_tick_at;
            gather_input_events();
            prev_simulation_tick_at = time::Instant::now();
            simulation(x, tick_rate);
        }

        let realnow2 = time::Instant::now();
        if realnow2 > next_render_frame_at {
            prev_render_frame_at = time::Instant::now();
            render();
        }
    }
}
