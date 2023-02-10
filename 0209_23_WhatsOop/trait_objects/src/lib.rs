pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// 这与定义使用了带有 trait bound 的泛型类型参数的结构体不同。
// 泛型类型参数一次只能替代一个具体类型，
// 而trait对象则允许在运行时替代多种具体类型。
impl Screen {
    pub fn run(&self) {
        for components in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 一个实现了 Draw trait 的 Button 结构体
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}