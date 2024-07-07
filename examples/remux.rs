fn main() {
    let mut input_container = rsav::open("data/sample.mov").unwrap();
    let mut output_container = rsav::create("data/sample_out.mp4").unwrap();

    let ist = input_container.streams().video().unwrap();
    let ost = output_container.add_stream_like(&ist);

    let ist_index = ist.index();
    let ost_index = ost.index();

    for (stream, mut packet) in input_container.demux() {
        if stream.index() == ist_index {
            packet.set_stream_index(ost_index);
            packet.set_pos(-1);
            output_container.mux(packet).unwrap();
        }
    }

    println!("Done!")
}
