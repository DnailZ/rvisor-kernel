use linux_module_rust

extern "C" {
    pub fn orig_getpid() -> c_long;

    fn protect_fs() -> bindings::mm_segment_t;
    fn release_fs(oldfs : bindings::mm_segment_t);
}