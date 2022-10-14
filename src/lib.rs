pub mod f32;

#[cfg(test)]
mod tests {
  use crate::f32::Vec3;

  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }

  #[test]
  fn cross_basis() {
    let result = Vec3::new(0f32, 0f32, 0f32);
    let a = Vec3::new(1f32, 2f32, 3f32);
    assert_eq!(a.cross(a), result);
  }
}
