use bench_util::setup_benchmark;
use time::ext::NumericalStdDurationShort;

setup_benchmark! {
    "Numerical durations (std, short)",

    fn unsigned(ben: &mut Bencher) {
        ben.iter(|| (
            5.nanoseconds(),
            5.microseconds(),
            5.milliseconds(),
            5.seconds(),
            5.minutes(),
            5.hours(),
            5.days(),
            5.weeks(),
        ));
    }

    fn float(ben: &mut Bencher) {
        ben.iter(|| (
            1.9.nanoseconds(),
            1.0.nanoseconds(),
            1.0.microseconds(),
            1.0.milliseconds(),
            1.0.seconds(),
            1.0.minutes(),
            1.0.hours(),
            1.0.days(),
            1.0.weeks(),
            1.5.nanoseconds(),
            1.5.microseconds(),
            1.5.milliseconds(),
            1.5.seconds(),
            1.5.minutes(),
            1.5.hours(),
            1.5.days(),
            1.5.weeks(),
        ));
    }
}
