use rand_distr::num_traits::ToPrimitive;
use std::time::Duration;

pub fn weighted_total<T: Copy + ToPrimitive>(iter: &[(Duration, T)]) -> f32 {
    let mut total = 0.0;
    let mut iter = iter.iter();
    let Some((mut current_time, mut value)) = iter.next() else {
        return 0.0;
    };
    for &(time, new_value) in iter {
        total += (time - current_time).as_secs_f32()
            * value.to_f32().expect("value should be convertible to f32");
        current_time = time;
        value = new_value;
    }
    total
}

pub fn duration<T>(iter: &[(Duration, T)]) -> Duration {
    let Some(&last) = iter.last().map(|(time, _)| time) else {
        return Duration::from_secs(0);
    };
    last
}

pub fn weighted_average<T: Copy + ToPrimitive>(iter: &[(Duration, T)]) -> f32 {
    let res = weighted_total(iter) / duration(iter).as_secs_f32();
    if res.is_nan() {
        return iter.last().map(|i| i.1.to_f32().unwrap_or(0.0)).unwrap_or(0.0);
    }
    res
}
