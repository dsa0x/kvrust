pub trait SomeTrait {
  fn is_valid(&self) -> bool;
}

#[derive(Debug)]
pub struct TestStr {
  pub name: String,
  pub age: i32,
  pub color: (i32, i32, i32),
}

pub fn test() -> TestStr {
  let mut test_str = TestStr {
    name: "sam".to_string(),
    age: 111,
    color: (255, 255, 255),
  };

  test_str.name = "cannotbe".to_string();
  println!("from fn: {:?}", test_str);

  test_str
}

impl SomeTrait for TestStr {
  fn is_valid(&self) -> bool {
    self.name.len() > 0
  }
}

impl TestStr {
  pub fn new(age: i32) -> Self {
    Self {
      name: "test".to_string(),
      age,
      color: (2, 2, 2),
    }
  }

  pub fn is_younger(&self, t2: Self) -> bool {
    self.age < t2.age
  }
}
