pub trait SomeTrait {
  fn some_function(&self) -> bool {
      true
  }
}

pub trait OtherTrait {
  fn other_function(&self) -> bool {
      true
  }
}

// 定义新特征，作为超特征
pub trait CombinedTrait: SomeTrait + OtherTrait {}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// 为 SomeStruct 和 OtherStruct 实现 CombinedTrait
impl CombinedTrait for SomeStruct {}
impl CombinedTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func(item: &(dyn CombinedTrait)) -> bool {
  item.some_function() && item.other_function()
}

fn main() {
  some_func(&SomeStruct {});
  some_func(&OtherStruct {});
}
