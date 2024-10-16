use math::trig::Trig;

#[test]
fn test_radians_90() {
    assert_eq!(0.03451456772742693, Trig::radians(90));
}

#[test]
fn test_radians_360() {
    assert_eq!(0.1380582709097077, Trig::radians(360));
}

#[test]
fn test_atan2_6144() {
    assert_eq!(6144, Trig::atan2(1, -1));
}

#[test]
fn test_atan2_12288() {
    assert_eq!(12288, Trig::atan2(-1, 0));
}

#[test]
fn test_sin() {
    let trig: Trig = Trig::new();
    assert_eq!(18, trig.sin(std::f64::consts::PI as i32));
}

#[test]
fn test_cos() {
    let trig: Trig = Trig::new();
    assert_eq!(16383, trig.cos(std::f64::consts::PI as i32));
}
