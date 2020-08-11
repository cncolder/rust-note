//! 另一种实现
//!
//! 3种类型没有特征约束了, 所以它们之间的方法签名不再统一.
//!
//! 这很不 OO.
//!
//! 使用 Rust 必须重新思考这个问题, 如何把 bug 尽可能消灭在编译时, 因为所有权这个新概念, OO 只是一种选择, 而不是最佳选择.
//!
//! 看完这篇文章, 不知你还是否认同 Rust 是不是一门面向对象语言.

/// 文章
pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

/// 草稿
pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost( {
            content: self.content,
        })
    }
}

/// 待审
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
