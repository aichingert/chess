use chess::run;

fn main() {
    pollster::block_on(run());
}
