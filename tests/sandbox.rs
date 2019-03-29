use probers::probe;
extern crate probers;

use probers_macros::prober;
#[prober]
trait TestProbes {
    fn probe0();
    fn probe1(foo: &str);
}

#[test]
pub fn probe_firing() {
    probe!(TestProbes::probe0());
    probe!(TestProbes::probe1("foo bar baz"));
}
