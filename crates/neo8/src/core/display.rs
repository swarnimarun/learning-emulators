pub struct Display<const W: usize, const H: usize> {
    v_buffer: [u8; W * H],
    maybe_dirty: bool
}