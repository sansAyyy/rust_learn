use cargo_project_organization::math;
use cargo_project_organization::models::User;

fn main() {
    // 模块用 mod 声明，用路径访问。
    // 路径可以从 crate 根开始，也可以从当前模块开始。

    let total = math::add(4, 6);
    println!("math::add(4, 6) = {total}");
    println!("public double = {}", math::double_for_public_api(5));

    let values = [10.0, 20.0, 30.0];
    let average = math::stats::average(&values).expect("values is not empty");
    println!("average = {average}");

    let user = User::new(7, "Bob");
    println!("user id = {}, name = {}", user.id(), user.name());

    // 可见性：
    // pub        对外公开。
    // pub(crate) 只在当前 crate 内公开。
    // 默认私有，只能在当前模块和子模块中使用。

    // math::internal_double(3); // 编译错误：这是 pub(crate)，外部 binary crate 不能调用。
}
