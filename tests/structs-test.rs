use smath::*;

#[test]
fn value_test(){
    let u_a:Value<u8>=Value::from(Some(0));
    let u_b:Value<u16>=Value::from(Some(1));
    let u_c:Value<u32>=Value::from_unwrapped(2);
    let u_d:Value<u64>=Value::from_unwrapped(3);
    let u_e:Value<u128>=Value::from(Some(4));
    let u_f:Value<usize>=Value::from(None);

    let i_a:Value<i8>=Value::from(Some(0));
    let i_b:Value<i16>=Value::from(Some(1));
    let i_c:Value<i32>=Value::from_unwrapped(2);
    let i_d:Value<i64>=Value::from_unwrapped(3);
    let i_e:Value<i128>=Value::from(Some(-1));
    let i_f:Value<isize>=Value::from(None);

    let f_a:Value<f32>=Value::from(Some(3.14));
    let f_b:Value<f64>=Value::none();

    assert_eq!(u_a.value(),Some(0));
    assert_ne!(u_b.value(),Some(100));
    assert_eq!(u_c.value_unwrapped(),2);
    assert_ne!(u_d.value_unwrapped(),100);
    assert_eq!(u_e.value_unwrapped_or(100),4);
    assert_eq!(u_f.value_unwrapped_or(100),100);

    assert_eq!(i_a.value(),Some(0));
    assert_ne!(i_b.value(),None);
    assert_eq!(i_c.value_unwrapped(),2);
    assert_ne!(i_d.value_unwrapped(),-100);
    assert_ne!(i_e.value_unwrapped_or(-100),-100);
    assert_ne!(i_f.value_unwrapped_or(-100),200);

    assert_eq!(f_a.value(),Some(3.14));
    assert_eq!(f_b.value(),None)
}

#[test]
fn scaled_angle_test(){
    let u_a:Angle<u8>=Angle::from_scaled(Some(0));
    let u_b:Angle<u16>=Angle::from_scaled(Some(1));
    let u_c:Angle<u32>=Angle::from_scaled_unwrapped(2);
    let u_d:Angle<u64>=Angle::from_scaled_unwrapped(3);
    let u_e:Angle<u128>=Angle::from_scaled(Some(4));
    let u_f:Angle<usize>=Angle::from_scaled(None);

    let i_a:Angle<i8>=Angle::from_scaled(Some(0));
    let i_b:Angle<i16>=Angle::from_scaled(Some(1));
    let i_c:Angle<i32>=Angle::from_scaled_unwrapped(2);
    let i_d:Angle<i64>=Angle::from_scaled_unwrapped(3);
    let i_e:Angle<i128>=Angle::from_scaled(Some(-1));
    let i_f:Angle<isize>=Angle::from_scaled(None);

    let f_a:Angle<f32>=Angle::from_scaled(Some(3.14));
    let f_b:Angle<f64>=Angle::none();

    assert_eq!(u_a.scaled_angle(),Some(0));
    assert_ne!(u_b.scaled_angle(),Some(100));
    assert_eq!(u_c.scaled_angle_unwrapped(),2);
    assert_ne!(u_d.scaled_angle_unwrapped(),100);
    assert_eq!(u_e.scaled_angle_unwrapped_or(100),4);
    assert_eq!(u_f.scaled_angle_unwrapped_or(100),100);

    assert_eq!(i_a.scaled_angle(),Some(0));
    assert_ne!(i_b.scaled_angle(),None);
    assert_eq!(i_c.scaled_angle_unwrapped(),2);
    assert_ne!(i_d.scaled_angle_unwrapped(),-100);
    assert_ne!(i_e.scaled_angle_unwrapped_or(-100),-100);
    assert_ne!(i_f.scaled_angle_unwrapped_or(-100),200);

    assert_eq!(f_a.scaled_angle(),Some(3.14));
    assert_eq!(f_b.scaled_angle(),None)
}

#[test]
fn value_angle_test(){}

#[test]
fn unit_test(){}

#[test]
fn value_unit_test(){}