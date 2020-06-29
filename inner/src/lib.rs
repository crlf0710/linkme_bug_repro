use linkme::distributed_slice;

#[distributed_slice]
pub static XXX: [fn()] = [..];

mod sub {
    #[linkme::distributed_slice(crate::XXX)]
    static YYY: fn() = real_fn;
    
    fn real_fn() {}    
}

pub fn dummy() -> usize {
    XXX.len()
}


#[test]
fn test1() {
    assert!(dummy() > 0);
}
