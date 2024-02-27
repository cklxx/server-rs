// 导入子模块
mod file;

// 在模块中定义函数
pub fn read_file_data() -> Vec<(String, String)> {
    println!("Hello from read_file_data!");

    // 调用子模块的函数
    let read_file_data = file::read_file_data();
    read_file_data
}
