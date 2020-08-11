use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("我午饭吃的凉菜");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("我午饭吃的凉菜", post.content());
}
