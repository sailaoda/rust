
// Post 结构体的定义和新建Post实例的new函数，State trait 和结构体 Draft
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        ""
    }

    // 实现Post和State trait 的 request_review方法
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self:Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    // self: Box<Self> 这个语法意味着该方法只可在持有这个类型的 Box 上被调用。
    // 这个语法获取了 Box<Self> 的所有权使老状态无效化，
    // 以便 Post 的状态值可转换为一个新状态。
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}